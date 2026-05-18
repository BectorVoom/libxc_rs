//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 693/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk693<F: Float>(t1423: F, t2493: F, t1447: F, t2489: F, t2481: F, t2485: F, t5220: F, t806: F, t2477: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6307 = t1423 * t2493;
    let t6308 = F::new(4.0) / F::new(135.0) * t6307;
    let t6309 = t1447 * t2489;
    let t6310 = F::new(4.0) / F::new(135.0) * t6309;
    let t6311 = t1423 * t2481;
    let t6312 = F::new(2.0) / F::new(135.0) * t6311;
    let t6313 = t1423 * t2485;
    let t6314 = F::new(2.0) / F::new(81.0) * t6313;
    let t6315 = t5220 * t806;
    let t6316 = F::new(4.0) / F::new(135.0) * t6315;
    let t6317 = t1423 * t2477;
    let t6318 = F::new(4.0) / F::new(135.0) * t6317;
    (t6307, t6308, t6309, t6310, t6311, t6312, t6313, t6314, t6315, t6316, t6317, t6318)
}
