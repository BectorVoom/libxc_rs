//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 918/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk918<F: Float>(t205: F, t6716: F, t208: F, t2414: F, t579: F, t213: F, t2526: F, t97: F, t588: F, t5376: F, t5379: F, t5388: F, t5393: F, t6657: F, t6677: F, t6690: F, t6692: F, t6694: F, t6707: F, t6709: F, t6711: F, t6715: F) -> (F, F, F, F, F) {
    let t6717 = t6716 * t205;
    let t6718 = t6717 * t208;
    let t6721 = t2414 * t579;
    let t6722 = t6721 * t208;
    let t6723 = t6722 * t213;
    let t6725 = t2526 * t97;
    let t6726 = t6725 * t588;
    let t6728 = t6657 + t6677 + t6690 + t6692 + t6694 + t6707 + t6709 + t6711 + F::new(2.0) / F::new(3.0) * t5376 + F::cast_from(0.2431111111111111_f64) * t5379 + t5388 + t5393 + t6715 + t6718 * t213 / F::new(3.0) + t6723 / F::new(3.0) + F::cast_from(0.06077777777777778_f64) * t6726;
    (t6717, t6718, t6721, t6722, t6728)
}
