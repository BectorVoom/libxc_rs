//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2959/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2959(t10904: f64, t17667: f64, t1040: f64, t17877: f64, t1041: f64, t1046: f64, t10517: f64, t10863: f64, t10898: f64, t13995: f64, t14235: f64, t17890: f64, t17962: f64, t248: f64, t3048: f64, t3062: f64, t3114: f64, t42522: f64, t42600: f64, t5857: f64, t5869: f64, t5875: f64, t5880: f64, t59676: f64, t61655: f64, t61659: f64, t61663: f64, t61665: f64) -> f64 {
    let t61675 = t10904 * t17667;
    let t61677 = t17877 * t1040;
    let t61686 = -t10898 * t5869 / 288.0_f64 + 5.0_f64 / 3456.0_f64 * t13995 * t14235 + t61655 / 2304.0_f64 - 19.0_f64 / 1728.0_f64 * t42600 * t5880 + t61659 / 1728.0_f64 - t61663 / 6912.0_f64 + t61665 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t1041 * t248 * t3062 * t59676 + t3114 * t17962 / 1536.0_f64 + 19.0_f64 / 864.0_f64 * t42522 * t5875 - t61675 / 216.0_f64 + t61677 * t1046 / 2304.0_f64 - t10863 * t5857 / 432.0_f64 - t3048 * t17890 / 432.0_f64 + 19.0_f64 / 1728.0_f64 * t10517 * t5869;
    t61686
}
