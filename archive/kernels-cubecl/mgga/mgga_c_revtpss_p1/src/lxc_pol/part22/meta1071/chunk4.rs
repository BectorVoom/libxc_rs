//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3840/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3840<F: Float>(t22076: F, t9962: F, t6861: F, t9994: F, t1353: F, t5658: F, t1398: F, t125: F, t22252: F, t124: F, t6843: F, t3938: F, t9816: F, t9818: F) -> (F, F, F, F, F, F, F) {
    let t73818 = t9962 * t22076;
    let t73820 = t6861 * t9994;
    let t73837 = t1353 * t5658;
    let t73842 = t6861 * t1398;
    let t73847 = t125 * t22252;
    let t73856 = t124 * t6843;
    let t73859 = t9816 * t9818 * t73856 * t3938;
    (t73818, t73820, t73837, t73842, t73847, t73856, t73859)
}
