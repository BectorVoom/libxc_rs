//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2206/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2206(t25580: f64, t3053: f64, t23529: f64, t4571: f64, t13961: f64, t6755: f64, t14202: f64, t6765: f64, t13950: f64, t14215: f64, t14491: f64, t1622: f64, t23454: f64, t3064: f64, t7578: f64, t82914: f64, t82941: f64, t82944: f64, t83016: f64, t83038: f64) -> f64 {
    let t88305 = t25580 * t3053 / 1728.0_f64;
    let t88307 = t23529 * t4571 / 324.0_f64;
    let t88320 = t6755 * t13961 / 1152.0_f64;
    let t88321 = t6765 * t14202;
    let t88324 = t6765 * t13950 / 1728.0_f64;
    let t88327 = t88305 - t88307 - 0.72670960969452703541e-2_f64 * t23454 * t7578 - t82914 / 3456.0_f64 + 0.20186378047070195428e-3_f64 * t82941 - 0.16149102437656156342e-2_f64 * t82944 + t6755 * t14491 / 1536.0_f64 + 5.0_f64 / 6912.0_f64 * t25580 * t3064 + t83016 * t14215 / 576.0_f64 + t88320 - t88321 / 10368.0_f64 + t88324 - t83038 * t1622 / 216.0_f64;
    t88327
}
