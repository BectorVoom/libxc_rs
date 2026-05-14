//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 933/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk933<F: Float>(t285: F, t477: F, t6138: F, t1191: F, t169: F, t2357: F, t301: F, t159: F, t2363: F, t39: F, t142: F, t774: F, t1549: F, t6093: F, t1: F, t1750: F, t1755: F, t2686: F) -> (F, F, F, F, F, F) {
    let t19850 = t6138 * t477 * t285;
    let t19860 = t169 * t1191 * t2357 * t301;
    let t19864 = t39 * t2363 * t159 * t285;
    let t19866 = t774 * t142;
    let t19872 = t1549 * t6093;
    let t19882 = t2686 * t1750 * t1 * t1755;
    (t19850, t19860, t19864, t19866, t19872, t19882)
}
