//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2601/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2601(t11697: f64, t22287: f64, t3577: f64, t15569: f64, t18371: f64, t1090: f64, t1216: f64, t15737: f64, t18303: f64, t18307: f64, t18364: f64, t18948: f64, t19051: f64, t21769: f64, t22244: f64, t3578: f64, t4950: f64, t4954: f64, t4989: f64, t52680: f64, t53083: f64, t53336: f64, t65803: f64, t66622: f64) -> f64 {
    let t72530 = t3577 * t11697 * t22287;
    let t72542 = t15569 * t18371;
    let t72552 = -5.0_f64 / 864.0_f64 * t15569 * t18364 - t3577 * t3578 * t21769 * t1216 / 768.0_f64 - t72530 / 1152.0_f64 - t53336 * t18303 / 32.0_f64 + t53083 * t18307 / 32.0_f64 + t65803 / 108.0_f64 - t52680 / 5184.0_f64 + 5.0_f64 / 4608.0_f64 * t19051 * t4989 + t15737 * t18948 / 256.0_f64 + t72542 / 216.0_f64 - 19.0_f64 / 864.0_f64 * t66622 * t4950 - t3577 * t3578 * t22244 * t1090 / 4608.0_f64 - 19.0_f64 / 864.0_f64 * t66622 * t4954;
    t72552
}
