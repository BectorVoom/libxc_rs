//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 918/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk918(t205: f64, t6716: f64, t208: f64, t2414: f64, t579: f64, t213: f64, t2526: f64, t97: f64, t588: f64, t5376: f64, t5379: f64, t5388: f64, t5393: f64, t6657: f64, t6677: f64, t6690: f64, t6692: f64, t6694: f64, t6707: f64, t6709: f64, t6711: f64, t6715: f64) -> (f64, f64, f64, f64, f64) {
    let t6717 = t6716 * t205;
    let t6718 = t6717 * t208;
    let t6721 = t2414 * t579;
    let t6722 = t6721 * t208;
    let t6723 = t6722 * t213;
    let t6725 = t2526 * t97;
    let t6726 = t6725 * t588;
    let t6728 = t6657 + t6677 + t6690 + t6692 + t6694 + t6707 + t6709 + t6711 + 2.0_f64 / 3.0_f64 * t5376 + 0.2431111111111111_f64 * t5379 + t5388 + t5393 + t6715 + t6718 * t213 / 3.0_f64 + t6723 / 3.0_f64 + 0.06077777777777778_f64 * t6726;
    (t6717, t6718, t6721, t6722, t6728)
}
