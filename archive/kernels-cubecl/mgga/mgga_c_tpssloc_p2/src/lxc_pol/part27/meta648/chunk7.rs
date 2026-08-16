//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2245/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2245<F: Float>(t10165: F, t1052: F, t1055: F, t13736: F, t1599: F, t1634: F, t1956: F, t23346: F, t23378: F, t23721: F, t23722: F, t25400: F, t25743: F, t25797: F, t3026: F, t3174: F, t3175: F, t4557: F, t4660: F, t50625: F, t6687: F, t6771: F, t7624: F, t83358: F, t83364: F, t83368: F, t83420: F, t88941: F, t88954: F, t89001: F, t89042: F, t89066: F, t89101: F, t89143: F, t89181: F, t89225: F, t89265: F, t89297: F, t89330: F, t89363: F, t89402: F, t89433: F, t89477: F, t89515: F, t89547: F) -> F {
    let t89556 = -F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t88941 * t25797 + F::cast_from(2.0_f64) * t4660 * t23378 - F::cast_from(6.0_f64) * t1052 * t10165 * t7624 * t3175 - F::cast_from(6.0_f64) * t6771 * t13736 - t88954 - t4557 * t23722 - t50625 * t1956 - F::cast_from(0.18277045187202515961e-2_f64) * t83358 + F::cast_from(0.54831135561607547884e-2_f64) * t83364 + F::cast_from(0.36554090374405031922e-2_f64) * t83368 + F::cast_from(2.0_f64) * t1052 * t3174 * t23721 * t1634 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t83420 + F::cast_from(4.0_f64) * t3026 * t25743 - t1052 * t1055 * (t89001 + t89042 + t89066 + t89101 + t89143 + t89181 + t89225 + t89265 + t89297 + t89330 + t89363 + t89402 + t89433 + t89477 + t89515 + t89547) + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25400;
    t89556
}
