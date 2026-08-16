//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2584/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2584(t1653: f64, t5011: f64, t19080: f64, t4997: f64, t1215: f64, t5398: f64, t11668: f64, t11678: f64, t11692: f64, t15569: f64, t15594: f64, t15659: f64, t1735: f64, t18236: f64, t18395: f64, t19016: f64, t22185: f64, t27524: f64, t3490: f64, t3577: f64, t3578: f64, t45119: f64, t4723: f64, t4729: f64, t475: f64, t52813: f64, t5971: f64, t6203: f64, t6230: f64, t6232: f64, t65424: f64, t65444: f64, t66388: f64) -> (f64, f64) {
    let t72146 = t1653 * t5011;
    let t72161 = t19080 * t4997;
    let t72164 = t5398 * t1215;
    let t72180 = t52813 * t6232 / 192.0_f64 + t65424 / 1536.0_f64 + 5.0_f64 / 4608.0_f64 * t15594 * t6203 + 5.0_f64 / 2304.0_f64 * t3490 * t22185 + t11692 * t3578 * t1735 * t72146 / 768.0_f64 - t45119 * t3578 * t66388 * t18395 / 1536.0_f64 + t11692 * t3578 * t6230 * t4729 / 768.0_f64 - 5.0_f64 / 432.0_f64 * t15569 * t19016 - t72161 / 144.0_f64 + t65444 / 432.0_f64 + 5.0_f64 / 4608.0_f64 * t3577 * t11668 * t4723 * t72164 * t475 - t3577 * t3578 * t18236 * t27524 * t475 / 768.0_f64 + 5.0_f64 / 2304.0_f64 * t11678 * t11668 * t15659 * t5971 * t1215;
    (t72146, t72180)
}
