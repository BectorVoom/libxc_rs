//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1248/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1248<F: Float>(t20799: F, t20803: F, t20805: F, t20810: F, t20813: F, t20816: F, t20818: F, t20820: F, t20822: F, t20824: F, t20828: F, t13707: F, t20831: F, t20833: F, t20834: F, t20836: F, t20843: F, t20845: F, t20847: F, t20849: F, t20852: F, t20854: F, t20856: F) -> (F, F) {
    let t22024 = t20799 - t20803 - t20805 + t20810 - t20813 - t20816 + t20818 + t20820 - t20822 - t20824 - t20828;
    let t22025 = -t20831 - t20833 + t20834 - t20836 - t20843 - t20845 - t20847 - t20849 + t13707 + t20852 + t20854 + t20856;
    (t22024, t22025)
}
