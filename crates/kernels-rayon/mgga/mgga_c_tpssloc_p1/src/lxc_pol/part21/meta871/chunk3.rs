//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3203/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3203(t11697: f64, t18386: f64, t3577: f64, t15608: f64, t15740: f64, t225: f64, t65165: f64, t1174: f64, t6183: f64, t698: f64, t11665: f64, t1216: f64, t15470: f64, t15474: f64, t15569: f64, t15700: f64, t18241: f64, t18383: f64, t18965: f64, t3578: f64, t45119: f64, t45134: f64, t45266: f64, t45296: f64, t484: f64, t488: f64, t4954: f64, t5012: f64, t52893: f64, t52897: f64, t53322: f64, t53515: f64, t53519: f64, t64874: f64, t68: f64) -> (f64, f64) {
    let t66646 = t3577 * t11697 * t18386;
    let t66648 = t15740 * t15608;
    let t66662 = t65165 * t225;
    let t66668 = t1174 * t698 * t6183;
    let t66670 = t15569 * t15470 / 216.0_f64 + t15569 * t15474 / 432.0_f64 - t11665 * t18383 / 2304.0_f64 - t3577 * t3578 * t18241 * t1216 / 2304.0_f64 - t45266 / 6912.0_f64 + t53515 / 5184.0_f64 + t53519 / 5184.0_f64 - t66646 / 1728.0_f64 - t66648 / 1728.0_f64 - t52893 * t3578 * t64874 / 192.0_f64 - t53322 * t4954 / 1152.0_f64 + t45119 * t52897 * t15700 * t5012 / 768.0_f64 + t45134 * t18965 / 2304.0_f64 - t45296 / 7776.0_f64 + t66662 * t68 * t484 * t488 / 3072.0_f64 + t66668 / 1296.0_f64;
    (t66662, t66670)
}
