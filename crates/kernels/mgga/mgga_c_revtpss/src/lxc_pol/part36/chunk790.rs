//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 790/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk790<F: Float>(t2482: F, t27: F, t2719: F, t820: F, t843: F, t821: F, t235: F, t239: F, t231: F, t2723: F, t2710: F, t826: F, t9732: F, t234: F, t2735: F, t2699: F, t798: F) -> (F, F, F, F, F, F, F, F) {
    let t10850 = t2482 * t2719 * t27;
    let t10858 = t820 * t2719 * t843;
    let t10866 = t821 * t821;
    let t10867 = 1.0 / t10866;
    let t10868 = t10867 * t235;
    let t10870 = t820 * t10868 * t239;
    let t10871 = t2723 * t231;
    let t10885 = 0.81322168495418382223e-4 * t2710 * t9732 * t826;
    let t10886 = t2735 * t234;
    let t10890 = t2699 * t798;
    (t10850, t10858, t10867, t10870, t10871, t10885, t10886, t10890)
}
