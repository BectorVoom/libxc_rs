//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1150/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1150<F: Float>(t10675: F, t10685: F, t10688: F, t10690: F, t10694: F, t10697: F, t10702: F, t10704: F, t10709: F, t14256: F, t23067: F, t23069: F, t23070: F, t10712: F, t10715: F, t10718: F, t10719: F, t14314: F, t14352: F, t14366: F, t23071: F, t23073: F, t23076: F, t23077: F, t23078: F, t23081: F, t23083: F) -> (F, F) {
    let t23340 = -t14256 - t23067 - t23069 - t23070 + t10675 + t10685 + 0.21642082724729686 * t10688 - 0.09618703433213194 * t10690 - t10694 + t10697 + 0.3246312408709453 * t10702 + 0.03354522822333102 * t10704 + t10709;
    let t23342 = t10712 - t10715 + t10718 - 0.011181742741110338 * t10719 + t23071 - t23073 + t14314 - t14352 - t23076 - t23077 + t23078 - t23081 - t23083 + t14366;
    (t23340, t23342)
}
