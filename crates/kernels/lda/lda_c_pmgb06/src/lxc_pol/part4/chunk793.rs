//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 793/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk793<F: Float>(t123: F, t2407: F, t317: F, t740: F, t2414: F, t395: F, t113: F, t301: F, t2448: F, t76: F, t1316: F, t1317: F, t2180: F, t2308: F, t2733: F, t2738: F, t2741: F, t329: F, t342: F, t346: F, t4398: F, t4414: F, t5569: F, t5573: F, t5578: F, t5580: F, t5721: F, t5934: F, t5937: F, t5939: F, t5980: F, t77: F, t790: F) -> (F, F, F) {
    let t5986 = t123 * t740 * t2407 * t317;
    let t5988 = t395 * t2414;
    let t5990 = t5988 * t113 * t301;
    let t5992 = t76 * t2448;
    let t5996 = 3.0 * t1316 * t2733 * t1317 - t346 * t2308 * t5934 + 0.019957056683757683 * t5937 + 6.0 * t5939 * t2738 + 6.0 * t1316 * t790 * t5721 + 6.0 * t1316 * t790 * t4414 - t346 * t4398 * t2741 + 0.002711962541669446 * t5569 + 0.39633663517353707 * t5573 - t5578 - 0.0011622696607154768 * t5580 + 3.0 * t329 * t77 * t5980 - 0.054045904796391424 * t5986 - 0.0002905674151788692 * t5990 + 6.0 * t2180 * t5992 * t342;
    (t5988, t5992, t5996)
}
