//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 768/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk768(t36364: f64, t787: f64, t1858: f64, t3601: f64, t6058: f64, t11595: f64, t769: f64, t11576: f64, t795: f64, t313: f64, t8748: f64, t1: f64, t36610: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36782 = t787 * t36364;
    let t36798 = t1858 * t3601;
    let t37032 = t6058 * t3601;
    let t37057 = t769 * t11595;
    let t37060 = t795 * t11576;
    let t37061 = t313 * t37060;
    let t37166 = t787 * t8748;
    let t37179 = t36610 * t1;
    (t36782, t36798, t37032, t37057, t37060, t37061, t37166, t37179)
}
