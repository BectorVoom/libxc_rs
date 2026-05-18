//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 853/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk853<F: Float>(t4398: F, t4401: F, t4403: F, t4406: F, t4408: F, t4412: F, t4416: F, t4418: F, t5690: F, t5695: F, t2696: F, t2699: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7327 = F::new(0.032530742648344574) * t4398;
    let t7328 = F::new(36.0) * t4401;
    let t7329 = F::new(96.0) * t4403;
    let t7330 = F::new(3.0) * t4406;
    let t7332 = F::new(60.0) * t4408;
    let t7333 = F::new(3.5089340384731225) * t4412;
    let t7334 = F::new(1.898172889849454) * t4416;
    let t7335 = F::new(2.0538164420033334) * t4418;
    let t7350 = F::new(24.0) * t5690;
    let t7353 = F::new(24.0) * t5695;
    let t8097 = F::new(1.8960024086108225) * t2696;
    let t8098 = F::new(0.06506148529668915) * t2699;
    (t7327, t7328, t7329, t7330, t7332, t7333, t7334, t7335, t7350, t7353, t8097, t8098)
}
