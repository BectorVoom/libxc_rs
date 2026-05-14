//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1257/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1257<F: Float>(t518: F, t6670: F, t577: F, t10675: F, t10680: F, t10685: F, t10686: F, t10688: F, t10690: F, t10694: F, t10697: F, t10699: F, t10702: F, t10704: F, t10709: F, t10712: F, t10715: F, t10718: F, t10719: F) -> (F, F) {
    let t18681 = t6670 * t518;
    let t18683 = 8.0 / 45.0 * t18681 * t577;
    let t18692 = t18683 + t10675 + 0.21642082724729686 * t10680 + t10685 + 0.07214027574909895 * t10686 + 0.4328416544945937 * t10688 - 0.19237406866426388 * t10690 - t10694 + t10697 + 0.011181742741110338 * t10699 + 0.6492624817418906 * t10702 + 0.06709045644666203 * t10704 + t10709 + t10712 - t10715 + t10718 - 0.022363485482220676 * t10719;
    (t18683, t18692)
}
