//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 871/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk871<F: Float>(t1124: F, t174: F, t318: F, t335: F, t1022: F, t1010: F, t386: F, t400: F, t2946: F, t8171: F, t1059: F, t2942: F) -> (F, F, F, F, F) {
    let t8427 = F::new(0.22161481481481482) * t174 * t1124 * t318 * t335;
    let t8428 = t1022 * t1022;
    let t8432 = F::new(3.5089340384731225) * t400 * t1010 * t8428 * t386;
    let t8437 = F::new(14.03573615389249) * t400 * t2946 * t8171 * t386;
    let t8438 = t1059 * t2942;
    (t8427, t8428, t8432, t8437, t8438)
}
