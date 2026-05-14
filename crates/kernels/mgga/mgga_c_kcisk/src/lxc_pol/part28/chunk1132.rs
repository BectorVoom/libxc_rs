//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1132/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1132<F: Float>(t32558: F, t32682: F, t233: F, t1065: F, t9406: F, t9645: F, t9660: F, t9657: F, t12261: F, t2784: F) -> (F, F, F, F, F, F) {
    let t32683 = t32558 + t32682;
    let t32684 = t233 * t32683;
    let t32685 = t1065 * t9406;
    let t32686 = 2.0 * t32685;
    let t32885 = t9645 * t9660;
    let t32887 = t9657 * t9660;
    let t32889 = t12261 * t2784;
    (t32683, t32684, t32686, t32885, t32887, t32889)
}
