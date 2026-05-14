//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 651/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk651<F: Float>(t1423: F, t2493: F, t1447: F, t2489: F, t2481: F, t2485: F, t5220: F, t806: F, t2477: F, t5194: F, t835: F, t2462: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6307 = t1423 * t2493;
    let t6308 = 4.0 / 135.0 * t6307;
    let t6309 = t1447 * t2489;
    let t6310 = 4.0 / 135.0 * t6309;
    let t6311 = t1423 * t2481;
    let t6312 = 2.0 / 135.0 * t6311;
    let t6313 = t1423 * t2485;
    let t6314 = 2.0 / 81.0 * t6313;
    let t6315 = t5220 * t806;
    let t6316 = 4.0 / 135.0 * t6315;
    let t6317 = t1423 * t2477;
    let t6318 = 4.0 / 135.0 * t6317;
    let t6319 = t5194 * t835;
    let t6320 = 4.0 / 135.0 * t6319;
    let t6321 = t1447 * t2462;
    let t6322 = 4.0 / 135.0 * t6321;
    (t6307, t6308, t6309, t6310, t6311, t6312, t6313, t6314, t6315, t6316, t6317, t6318, t6319, t6320, t6321, t6322)
}
