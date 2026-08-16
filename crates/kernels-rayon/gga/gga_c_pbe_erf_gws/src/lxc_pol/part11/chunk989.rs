//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 989/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk989(t12041: f64, t19894: f64, t1114: f64, t4383: f64, t9847: f64, t4473: f64, t4384: f64, t3916: f64, t19898: f64, t3912: f64, t6154: f64, t3747: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34773 = t12041 * t19894;
    let t34850 = t1114 * t9847 * t4383;
    let t34857 = t12041 * t4473;
    let t34914 = t12041 * t4384;
    let t34922 = t3916 * t4384;
    let t35000 = t3912 * t19898;
    let t35003 = t3912 * t4384;
    let t35014 = t3912 * t6154;
    let t35057 = t1114 * t3747 * t4383;
    (t34773, t34850, t34857, t34914, t34922, t35000, t35003, t35014, t35057)
}
