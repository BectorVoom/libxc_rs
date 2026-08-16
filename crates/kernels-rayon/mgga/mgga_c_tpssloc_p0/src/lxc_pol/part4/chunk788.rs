//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 788/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk788(t5742: f64, t932: f64, t2868: f64, t2875: f64, t4335: f64, t4384: f64, t5679: f64, t5683: f64, t5687: f64, t5699: f64, t5706: f64, t5712: f64, t5714: f64, t5718: f64, t5721: f64, t5724: f64) -> (f64, f64) {
    let t5743 = t5742 * t932;
    let t5758 = -0.17648625e1_f64 * t5699 + 0.3529725e1_f64 * t5706 + t2868 + 0.34431666666666666666e0_f64 * t4335 - 0.34431666666666666667e0_f64 * t5679 + 0.103295e1_f64 * t5683 - 0.516475e0_f64 * t5687 + 0.31558125e0_f64 * t5712 + 0.6311625e0_f64 * t5714 + t2875 + 0.13892666666666666667e0_f64 * t4384 - 0.34731666666666666667e-1_f64 * t5718 + 0.20839e0_f64 * t5721 - 0.104195e0_f64 * t5724;
    (t5743, t5758)
}
