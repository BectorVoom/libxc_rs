//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2245/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2245(t10165: f64, t1052: f64, t1055: f64, t13736: f64, t1599: f64, t1634: f64, t1956: f64, t23346: f64, t23378: f64, t23721: f64, t23722: f64, t25400: f64, t25743: f64, t25797: f64, t3026: f64, t3174: f64, t3175: f64, t4557: f64, t4660: f64, t50625: f64, t6687: f64, t6771: f64, t7624: f64, t83358: f64, t83364: f64, t83368: f64, t83420: f64, t88941: f64, t88954: f64, t89001: f64, t89042: f64, t89066: f64, t89101: f64, t89143: f64, t89181: f64, t89225: f64, t89265: f64, t89297: f64, t89330: f64, t89363: f64, t89402: f64, t89433: f64, t89477: f64, t89515: f64, t89547: f64) -> f64 {
    let t89556 = -0.82246703342411321825e-2_f64 * t6687 * t88941 * t25797 + 2.0_f64 * t4660 * t23378 - 6.0_f64 * t1052 * t10165 * t7624 * t3175 - 6.0_f64 * t6771 * t13736 - t88954 - t4557 * t23722 - t50625 * t1956 - 0.18277045187202515961e-2_f64 * t83358 + 0.54831135561607547884e-2_f64 * t83364 + 0.36554090374405031922e-2_f64 * t83368 + 2.0_f64 * t1052 * t3174 * t23721 * t1634 + 0.16449340668482264365e-1_f64 * t6687 * t1599 * t83420 + 4.0_f64 * t3026 * t25743 - t1052 * t1055 * (t89001 + t89042 + t89066 + t89101 + t89143 + t89181 + t89225 + t89265 + t89297 + t89330 + t89363 + t89402 + t89433 + t89477 + t89515 + t89547) + 0.43864908449286038306e-1_f64 * t23346 * t25400;
    t89556
}
