//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 273/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk273(t247: f64, t780: f64, t251: f64, t108: f64, t659: f64, t661: f64, t739: f64, t743: f64, t256: f64, t267: f64, t517: f64, t570: f64, t649: f64, t655: f64, t658: f64, t670: f64, t788: f64, t797: f64, t801: f64, t810: f64, t815: f64, t824: f64, t828: f64, t837: f64) -> (f64, f64, f64, f64) {
    let t850 = t780 * t247;
    let t851 = t850 * t251;
    let t858 = (4.0_f64 / 3.0_f64 * t659 * t739 + 4.0_f64 / 3.0_f64 * t661 * t743) * t108;
    let t861 = t788 + t797 + t517 + t801 - t810 + t815 + t824 + t570 + t828 - t837 + t851 * t256 / 3.0_f64 + t649 + t655 + t658 - t858 * t267 / 15.0_f64 - t670;
    (t850, t851, t858, t861)
}
