//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 744/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk744(t1689: f64, t2997: f64, t1038: f64, t1431: f64, t2996: f64, t128: f64, t644: f64, t640: f64) -> (f64, f64, f64, f64) {
    let t8763 = t2997 * t1689;
    let t8764 = t1038 * t1431;
    let t8765 = t8763 * t8764;
    let t8766 = t2996 * t8765;
    let t8768 = t128 * t644;
    let t8769 = t640 * t8768;
    (t8765, t8766, t8768, t8769)
}
