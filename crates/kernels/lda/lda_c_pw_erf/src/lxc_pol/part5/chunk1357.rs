//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1357/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1357<F: Float>(t10675: F, t10685: F, t10688: F, t10690: F, t10694: F, t10697: F, t10702: F, t10704: F, t10709: F, t14256: F, t23067: F, t23069: F, t23070: F) -> F {
    let t23340 = -t14256 - t23067 - t23069 - t23070 + t10675 + t10685 + F::new(0.21642082724729686) * t10688 - F::new(0.09618703433213194) * t10690 - t10694 + t10697 + F::new(0.3246312408709453) * t10702 + F::new(0.03354522822333102) * t10704 + t10709;
    t23340
}
