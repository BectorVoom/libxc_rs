//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3018/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3018(t18053: f64, t225: f64, t4693: f64, t10160: f64, t1049: f64, t1052: f64, t1055: f64, t1066: f64, t11010: f64, t14529: f64, t14552: f64, t14555: f64, t14659: f64, t17588: f64, t17875: f64, t18047: f64, t18166: f64, t3020: f64, t3026: f64, t3174: f64, t3207: f64, t349: f64, t388: f64, t4557: f64, t4665: f64, t4694: f64, t5914: f64, t5920: f64, t61646: f64, t62914: f64, t62953: f64, t62988: f64, t63022: f64, t63058: f64, t63095: f64, t63133: f64, t63168: f64, t63198: f64, t990: f64) -> f64 {
    let t63215 = t18053 * t225;
    let t63220 = t4693 * t4693;
    let t63235 = 8.0_f64 * t14529 * t4665 - 2.0_f64 * t61646 * t1066 + t349 * t62914 * t388 - t1052 * t1055 * (t62953 + t62988 + t63022 + t63058 + t63095 + t63133 + t63168 + t63198) + 2.0_f64 * t17875 * t1049 * t388 + t3020 * t5914 * t388 + 8.0_f64 * t14555 * t4665 - 2.0_f64 * t3026 * t18166 - 4.0_f64 * t14552 * t4694 - 2.0_f64 * t63215 * t1066 + 8.0_f64 * t14552 * t4665 + 4.0_f64 * t1052 * t3174 * t63220 - 2.0_f64 * t4557 * t14659 + 2.0_f64 * t11010 * t5920 - 2.0_f64 * t17588 * t3207 + 2.0_f64 * t990 * t18047 * t388 + 4.0_f64 * t10160 * t5920;
    t63235
}
