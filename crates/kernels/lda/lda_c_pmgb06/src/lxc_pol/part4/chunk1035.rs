//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1035/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1035<F: Float>(t187: F, t3024: F, t3389: F, t534: F, t540: F, t3018: F, t3027: F, t1179: F, t186: F, t543: F, t55: F, t27: F, t545: F) -> (F, F, F, F, F, F, F) {
    let t10690 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t3024 * t187;
    let t10693 = t534 * t3389;
    let t10696 = F::cast_from(0.004413481481481482_f64) * t540 * t3389;
    let t10697 = t3018 * t187;
    let t10699 = t3027 * t187;
    let t10711 = F::cast_from(0.2244364134416412_f64) * t543 * t55 * t1179 * t186;
    let t10714 = F::cast_from(0.4328416544945937_f64) * t3024 * t27 * t545;
    (t10690, t10693, t10696, t10697, t10699, t10711, t10714)
}
