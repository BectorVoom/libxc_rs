//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 824/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk824<F: Float>(t10866: F, t235: F, t239: F, t820: F, t231: F, t2723: F, t10665: F, t827: F, t828: F, t10666: F, t2648: F, t2741: F, t2710: F, t826: F, t9732: F, t234: F, t2735: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10867 = 1.0 / t10866;
    let t10868 = t10867 * t235;
    let t10870 = t820 * t10868 * t239;
    let t10871 = t2723 * t231;
    let t10872 = t10665 * t10871;
    let t10874 = t827 * t828 * t10872;
    let t10878 = t827 * t828 * t10666;
    let t10881 = t2741 * t2648;
    let t10885 = 0.81322168495418382223e-4 * t2710 * t9732 * t826;
    let t10886 = t2735 * t234;
    (t10867, t10870, t10871, t10872, t10874, t10878, t10881, t10885, t10886)
}
