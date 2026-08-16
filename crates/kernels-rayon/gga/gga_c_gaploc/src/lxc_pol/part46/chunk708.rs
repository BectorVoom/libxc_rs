//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 708/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk708(t13034: f64, t5748: f64, t2949: f64, t3209: f64, t1445: f64, t813: f64, t2958: f64, t833: f64, t2097: f64, t3039: f64, t3277: f64, t13010: f64, t13015: f64, t13018: f64, t13021: f64, t13026: f64, t13028: f64, t13029: f64, t13031: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13036 = 0.27606906686822939767e2_f64 * t5748 * t13034;
    let t13037 = t2949 * t3209;
    let t13038 = t1445 * t13037;
    let t13040 = 0.92023022289409799224e1_f64 * t813 * t13038;
    let t13041 = t2958 * t3209;
    let t13042 = t1445 * t13041;
    let t13044 = 0.43710935587469654631e2_f64 * t833 * t13042;
    let t13045 = t3039 * t2097;
    let t13047 = 0.25025342966295298669e1_f64 * t3277 * t13045;
    let t13048 = -0.13803453343411469884e2_f64 * t13010 - t13015 - t13018 + 0.14300195980740170668e1_f64 * t13021 + t13026 + t13028 + 0.71500979903700853338e0_f64 * t13029 - 0.21450293971110256002e1_f64 * t13031 + t13036 - t13040 + t13044 - t13047;
    (t13037, t13038, t13041, t13042, t13045, t13048)
}
