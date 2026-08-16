//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3209/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3209(t19253: f64, t225: f64, t5088: f64, t11925: f64, t1238: f64, t1241: f64, t1251: f64, t1252: f64, t14980: f64, t15786: f64, t15803: f64, t15820: f64, t1760: f64, t1761: f64, t19208: f64, t19220: f64, t19234: f64, t3593: f64, t3598: f64, t3599: f64, t3631: f64, t45350: f64, t466: f64, t498: f64, t5055: f64, t5060: f64, t51925: f64, t51928: f64, t6243: f64, t6268: f64, t65208: f64, t65249: f64, t65343: f64, t65374: f64, t65408: f64, t66675: f64, t66702: f64, t66737: f64, t66769: f64, t66802: f64) -> f64 {
    let t66822 = t19253 * t225;
    let t66825 = t5088 * t5088;
    let t66842 = -2.0_f64 * t65208 * t1252 + 4.0_f64 * t5055 * t15803 - t1238 * t1241 * (t65249 + t65343 + t65374 + t65408 + t66702 + t66737 + t66769 + t66802) + 24.0_f64 * t1238 * t45350 * t6243 * t3599 + 8.0_f64 * t15820 * t5060 - 2.0_f64 * t19234 * t3631 + 4.0_f64 * t1238 * t3598 * t19208 * t1251 + 8.0_f64 * t14980 * t5060 - 4.0_f64 * t66822 * t1252 + 4.0_f64 * t1238 * t3598 * t66825 + t466 * t66675 * t498 - 2.0_f64 * t51928 * t1761 - t11925 * t6268 + 4.0_f64 * t3593 * t19220 + 4.0_f64 * t1238 * t3598 * t1760 * t15786 - 4.0_f64 * t51925 * t1761;
    t66842
}
