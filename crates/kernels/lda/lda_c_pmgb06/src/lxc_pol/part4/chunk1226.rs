//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1226/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1226<F: Float>(t17640: F, t17643: F, t17647: F, t17650: F, t17652: F, t17653: F, t17655: F, t17657: F, t17661: F, t17662: F, t17665: F, t17667: F, t17669: F, t17674: F, t17675: F, t17676: F, t17680: F, t17682: F, t17683: F, t17684: F, t17686: F, t17688: F, t17691: F, t17693: F, t17695: F, t17697: F, t17699: F, t17702: F, t17703: F, t17704: F) -> (F, F) {
    let t18318 = -t17640 + t17643 - t17647 - t17650 + t17652 - t17653 - t17655 - t17657 + t17661 + t17662 + t17665 + t17667 - t17669 + t17674 - t17675;
    let t18319 = -t17676 - t17680 + t17682 - t17683 + t17684 + t17686 + t17688 - t17691 + t17693 - t17695 + t17697 + t17699 + t17702 + t17703 + t17704;
    (t18318, t18319)
}
