//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1114/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1114(t1636: f64, t8240: f64, t2268: f64, t6256: f64, t8251: f64, t2128: f64, t8010: f64, t28583: f64, t28585: f64, t28587: f64, t28590: f64, t28592: f64, t28595: f64, t28598: f64, t28600: f64, t28602: f64, t28604: f64, t28606: f64, t28608: f64, t28611: f64) -> (f64, f64, f64, f64, f64) {
    let t28655 = t8240 * t1636;
    let t28658 = t2268 * t6256;
    let t28663 = t8251 * t1636;
    let t28666 = t8010 * t2128;
    let t28683 = -0.10791666666666666667e0_f64 * t28583 + 0.20234375e-1_f64 * t28585 + 0.14388888888888888889e0_f64 * t28587 - 0.9375e-1_f64 * t28590 - 0.20234375e-1_f64 * t28592 + 0.25e0_f64 * t28595 - 0.9375e-1_f64 * t28598 + 0.20234375e-1_f64 * t28600 + 0.1875e0_f64 * t28602 - 0.26979166666666666667e-1_f64 * t28604 - 0.625e-1_f64 * t28606 - 0.26979166666666666667e-1_f64 * t28608 - 0.16666666666666666667e0_f64 * t28611;
    (t28655, t28658, t28663, t28666, t28683)
}
