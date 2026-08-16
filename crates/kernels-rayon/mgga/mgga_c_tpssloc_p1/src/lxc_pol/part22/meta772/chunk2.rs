//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2634/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2634(t225: f64, t73575: f64, t11665: f64, t11668: f64, t11678: f64, t1215: f64, t15659: f64, t19083: f64, t22162: f64, t3577: f64, t3578: f64, t45296: f64, t484: f64, t488: f64, t4965: f64, t4974: f64, t5012: f64, t52893: f64, t53516: f64, t53520: f64, t5975: f64, t5979: f64, t6164: f64, t66646: f64, t66648: f64, t66668: f64, t68: f64, t73138: f64, t73142: f64) -> (f64, f64) {
    let t73576 = t73575 * t225;
    let t73587 = -t11678 * t3578 * t15659 * t5975 * t1215 / 384.0_f64 - t3577 * t3578 * t5012 * t5979 / 1536.0_f64 + t19083 * t4974 / 72.0_f64 + 5.0_f64 / 768.0_f64 * t52893 * t11668 * t73138 - 77.0_f64 / 486.0_f64 * t73142 + 19.0_f64 / 576.0_f64 * t4965 * t6164 * t488 + t73576 * t68 * t484 * t488 / 3072.0_f64 + t53516 + t53520 - t66646 / 1152.0_f64 - t66648 / 1152.0_f64 - t11665 * t22162 / 1536.0_f64 - t45296 / 15552.0_f64 + t66668 / 432.0_f64;
    (t73576, t73587)
}
