//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 997/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk997(t1188: f64, t12943: f64, t14849: f64, t1588: f64, t16095: f64, t16097: f64, t16099: f64, t17733: f64, t17746: f64, t17750: f64, t17753: f64, t17853: f64, t18178: f64, t18184: f64, t18188: f64, t18191: f64, t18194: f64, t18200: f64, t18205: f64, t18214: f64, t18218: f64, t277: f64, t4281: f64, t4297: f64, t490: f64, t5229: f64, t5246: f64, t9254: f64, t95: f64) -> f64 {
    let t18223 = 0.25844881434903430496e-2_f64 * t95 * t277 * t18178 * t1188 - t17746 + 2.0_f64 / 3.0_f64 * t4281 * t18184 - t4281 * t18188 + 200.0_f64 / 81.0_f64 * t4297 * t18191 - 50.0_f64 * t18194 * t1588 - 380000.0_f64 / 81.0_f64 * t18200 * t5246 + t17853 - 616.0_f64 / 27.0_f64 * t490 * t18205 - t17753 + 2.0_f64 / 9.0_f64 * t16095 - 8.0_f64 / 9.0_f64 * t16097 + 100.0_f64 / 27.0_f64 * t14849 * t5229 - 8.0_f64 / 3.0_f64 * t16099 + 50.0_f64 / 27.0_f64 * t4297 * t18214 + t17750 - t17733 - t12943 / 3.0_f64 + 0.51689762869806860992e-2_f64 * t95 * t277 * t18218 * t9254;
    t18223
}
