//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 841/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk841(t228: f64, t2670: f64, t163: f64, t661: f64, t660: f64, t203: f64, t985: f64, t202: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11797 = t2670 * t228;
    let t11799 = t661 * t163;
    let t11800 = t660 * t11799;
    let t11802 = t203 * t985;
    let t11803 = t202 * t11802;
    let t11805 = t6 * t985;
    (t11797, t11799, t11800, t11802, t11803, t11805)
}
