//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1854/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1854<F: Float>(t7063: F, t94878: F, t25877: F, t94801: F, t1419: F, t786: F, t2453: F, t25949: F, t25898: F, t112: F, t843: F, t239: F, t655: F) -> (F, F, F, F, F, F, F, F) {
    let t94879 = t7063 * t94878;
    let t94886 = t94801 * t25877;
    let t94889 = t786 * t1419;
    let t94890 = t94889 * t25877;
    let t94894 = t786 * t94878;
    let t94913 = t2453 * t25949;
    let t94921 = t94889 * t25898;
    let t94973 = t843 * t112;
    let t94975 = t239 * t655;
    (t94879, t94886, t94890, t94894, t94913, t94921, t94973, t94975)
}
