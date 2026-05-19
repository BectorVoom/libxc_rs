//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1114/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1114<F: Float>(t1636: F, t8240: F, t2268: F, t6256: F, t8251: F, t2128: F, t8010: F, t28583: F, t28585: F, t28587: F, t28590: F, t28592: F, t28595: F, t28598: F, t28600: F, t28602: F, t28604: F, t28606: F, t28608: F, t28611: F) -> (F, F, F, F, F) {
    let t28655 = t8240 * t1636;
    let t28658 = t2268 * t6256;
    let t28663 = t8251 * t1636;
    let t28666 = t8010 * t2128;
    let t28683 = -F::cast_from(0.10791666666666666667e0_f64) * t28583 + F::new(0.20234375e-1) * t28585 + F::cast_from(0.14388888888888888889e0_f64) * t28587 - F::new(0.9375e-1) * t28590 - F::new(0.20234375e-1) * t28592 + F::new(0.25e0) * t28595 - F::new(0.9375e-1) * t28598 + F::new(0.20234375e-1) * t28600 + F::new(0.1875e0) * t28602 - F::cast_from(0.26979166666666666667e-1_f64) * t28604 - F::new(0.625e-1) * t28606 - F::cast_from(0.26979166666666666667e-1_f64) * t28608 - F::cast_from(0.16666666666666666667e0_f64) * t28611;
    (t28655, t28658, t28663, t28666, t28683)
}
