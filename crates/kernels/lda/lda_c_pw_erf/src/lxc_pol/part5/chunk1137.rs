//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1137/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1137<F: Float>(t21739: F, t21740: F, t21741: F, t21743: F, t21746: F, t21750: F, t21752: F, t21754: F, t21757: F, t21761: F, t21764: F, t21766: F, t21767: F, t13420: F, t17814: F, t17816: F, t17820: F, t21771: F, t21775: F, t21776: F, t21871: F, t21875: F, t21878: F, t21881: F, t21885: F, t21888: F) -> (F, F) {
    let t23271 = -t21739 + t21740 + t21741 + t21743 + t21746 - t21750 - t21752 - t21754 - t21757 + t21761 + t21764 + t21766 + t21767;
    let t23275 = t21771 + t21775 - t21776 + t21871 - t13420 + 0.6492624817418906 * t17814 + 0.21642082724729686 * t17816 + 0.3246312408709453 * t17820 - t21875 + t21878 - t21881 - t21885 + t21888;
    (t23271, t23275)
}
