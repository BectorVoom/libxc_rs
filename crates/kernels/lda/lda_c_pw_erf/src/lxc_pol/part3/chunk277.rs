//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 277/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk277<F: Float>(t247: F, t780: F, t251: F, t108: F, t659: F, t661: F, t739: F, t743: F, t256: F, t267: F, t517: F, t570: F, t649: F, t655: F, t658: F, t670: F, t788: F, t797: F, t801: F, t810: F, t815: F, t824: F, t828: F, t837: F) -> (F, F, F, F) {
    let t850 = t780 * t247;
    let t851 = t850 * t251;
    let t858 = (F::new(4.0) / F::new(3.0) * t659 * t739 + F::new(4.0) / F::new(3.0) * t661 * t743) * t108;
    let t861 = t788 + t797 + t517 + t801 - t810 + t815 + t824 + t570 + t828 - t837 + t851 * t256 / F::new(3.0) + t649 + t655 + t658 - t858 * t267 / F::new(15.0) - t670;
    (t850, t851, t858, t861)
}
