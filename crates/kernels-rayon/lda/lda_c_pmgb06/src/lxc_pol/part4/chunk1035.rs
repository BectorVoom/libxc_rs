//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1035/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1035(t187: f64, t3024: f64, t3389: f64, t534: f64, t540: f64, t3018: f64, t3027: f64, t1179: f64, t186: f64, t543: f64, t55: f64, t27: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10690 = 16.0_f64 / 3.0_f64 * t3024 * t187;
    let t10693 = t534 * t3389;
    let t10696 = 0.004413481481481482_f64 * t540 * t3389;
    let t10697 = t3018 * t187;
    let t10699 = t3027 * t187;
    let t10711 = 0.2244364134416412_f64 * t543 * t55 * t1179 * t186;
    let t10714 = 0.4328416544945937_f64 * t3024 * t27 * t545;
    (t10690, t10693, t10696, t10697, t10699, t10711, t10714)
}
