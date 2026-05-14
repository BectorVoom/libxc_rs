//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1019/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1019<F: Float>(t1419: F, t786: F, t25877: F, t94878: F, t2453: F, t25949: F, t25898: F, t112: F, t843: F, t239: F, t655: F, t665: F, t2339: F, t624: F, t2340: F, t2366: F, t25823: F) -> (F, F, F, F, F, F, F, F) {
    let t94889 = t786 * t1419;
    let t94890 = t94889 * t25877;
    let t94894 = t786 * t94878;
    let t94913 = t2453 * t25949;
    let t94921 = t94889 * t25898;
    let t94973 = t843 * t112;
    let t94975 = t239 * t655;
    let t94976 = t94975 * t665;
    let t94978 = t624 * t2339;
    let t94979 = t94978 * t2340;
    let t94981 = t25823 * t2366;
    (t94890, t94894, t94913, t94921, t94973, t94976, t94979, t94981)
}
