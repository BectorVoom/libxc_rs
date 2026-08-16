//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3008/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3008(t10403: f64, t10422: f64, t18015: f64, t1036: f64, t18010: f64, t14025: f64, t14508: f64, t13970: f64, t14511: f64, t10263: f64, t10408: f64, t13546: f64, t14222: f64, t14228: f64, t1616: f64, t17156: f64, t17637: f64, t17643: f64, t3048: f64, t3070: f64, t3071: f64, t3088: f64, t3151: f64, t378: f64, t43382: f64, t49934: f64, t50438: f64, t50442: f64, t55723: f64, t5885: f64, t5890: f64, t5904: f64, t973: f64, t974: f64) -> f64 {
    let t62891 = t10403 * t10422 * t18015;
    let t62893 = t18010 * t1036;
    let t62901 = t14508 * t14025;
    let t62903 = t14511 * t13970;
    let t62909 = -t49934 * t14222 / 1152.0_f64 - t3070 * t3071 * t1616 * t13546 / 1152.0_f64 - t973 * t974 * t3151 * t55723 / 72.0_f64 + 11.0_f64 / 324.0_f64 * t10263 * t5890 - 11.0_f64 / 162.0_f64 * t10263 * t5885 - 5.0_f64 / 1152.0_f64 * t3070 * t10408 * t17156 * t14228 + t62891 / 864.0_f64 - t62893 / 432.0_f64 + 19.0_f64 / 1728.0_f64 * t5904 * t3088 * t378 + t43382 / 5184.0_f64 + t50438 / 1728.0_f64 + t50442 / 972.0_f64 + t62901 / 576.0_f64 - t62903 / 1152.0_f64 + t3048 * t17637 / 216.0_f64 - 5.0_f64 / 1296.0_f64 * t3048 * t17643;
    t62909
}
