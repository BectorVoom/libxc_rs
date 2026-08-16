//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 727/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk727(t2001: f64, t3854: f64, t1318: f64, t3804: f64, t3856: f64, t3861: f64, t3865: f64, t1511: f64, t793: f64, t184: f64, t199: f64, t1519: f64, t795: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4581 = t3854 * t2001;
    let t4583 = 32.0_f64 / 135.0_f64 * t1318 * t4581;
    let t4584 = 16.0_f64 / 135.0_f64 * t3804;
    let t4585 = 32.0_f64 / 135.0_f64 * t3856;
    let t4586 = 32.0_f64 / 135.0_f64 * t3861;
    let t4587 = 16.0_f64 / 135.0_f64 * t3865;
    let t4588 = t1511 * t793;
    let t4589 = t4588 * t184;
    let t4591 = 4.0_f64 / 15.0_f64 * t4589 * t199;
    let t4592 = t795 * t1519;
    (t4581, t4583, t4584, t4585, t4586, t4587, t4588, t4589, t4591, t4592)
}
