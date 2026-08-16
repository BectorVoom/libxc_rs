//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3165/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3165(t1653: f64, t3507: f64, t11678: f64, t11697: f64, t19001: f64, t11692: f64, t11825: f64, t14726: f64, t15659: f64, t15661: f64, t15702: f64, t1735: f64, t18395: f64, t19083: f64, t19101: f64, t3490: f64, t3493: f64, t3577: f64, t3578: f64, t3587: f64, t45114: f64, t45128: f64, t45197: f64, t52704: f64, t53149: f64, t6207: f64, t65464: f64, t65469: f64, t65474: f64, t65479: f64, t65482: f64, t65485: f64) -> f64 {
    let t65492 = t1653 * t3507;
    let t65506 = t11678 * t11697 * t19001;
    let t65518 = -t11678 * t3578 * t65464 * t15661 / 1152.0_f64 + t11692 * t3578 * t65469 * t15702 / 2304.0_f64 - t45197 * t3578 * t65474 * t15661 / 384.0_f64 - t65479 / 1728.0_f64 + t65482 / 1728.0_f64 - t65485 / 864.0_f64 - t11678 * t3578 * t15659 * t1653 * t3493 / 1152.0_f64 - t45197 * t3578 * t52704 * t65492 / 384.0_f64 + t45114 * t3578 * t15659 * t65492 / 384.0_f64 + t11692 * t3578 * t53149 * t18395 / 2304.0_f64 - t65506 / 864.0_f64 - 5.0_f64 / 2592.0_f64 * t3577 * t45128 * t1735 * t14726 - 5.0_f64 / 1296.0_f64 * t19083 * t3587 - t11825 * t6207 / 4608.0_f64 - t3490 * t19101 / 2304.0_f64;
    t65518
}
