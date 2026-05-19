//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 919/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk919<F: Float>(t113: F, t1798: F, t247: F, t301: F, t1147: F, t123: F, t2164: F, t317: F, t2257: F, t26: F, t329: F, t413: F, t5567: F) -> (F, F, F, F) {
    let t11628 = t247 * t1798 * t113 * t301;
    let t11629 = F::cast_from(0.004067943812504169_f64) * t11628;
    let t11632 = t123 * t1147 * t2164 * t317;
    let t11633 = F::cast_from(0.5945049527603057_f64) * t11632;
    let t11639 = t26 * t2257;
    let t11640 = t329 * t11639;
    let t11674 = t5567 * t413 * t301;
    (t11629, t11633, t11640, t11674)
}
