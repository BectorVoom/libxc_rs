//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 115/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk115<F: Float>(t103: F, t235: F, t36: F, t37: F) -> (F, F, F) {
    let t265 = F::new(7.05945) * t37 + F::new(1.549425) * t36 + F::new(0.420775) * t235 + F::new(0.1562925) * t103;
    let t268 = F::new(1.0) + F::cast_from(32.16395899738507_f64) / t265;
    let t269 = F::ln(t268);
    (t265, t268, t269)
}
