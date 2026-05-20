//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 918/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk918<F: Float>(t10861: F, t827: F, t828: F, t821: F, t235: F, t239: F, t820: F, t231: F, t2723: F, t10665: F, t10666: F, t2648: F, t2741: F) -> (F, F, F, F, F, F, F, F) {
    let t10863 = t827 * t828 * t10861;
    let t10866 = t821 * t821;
    let t10867 = F::new(1.0) / t10866;
    let t10868 = t10867 * t235;
    let t10870 = t820 * t10868 * t239;
    let t10871 = t2723 * t231;
    let t10872 = t10665 * t10871;
    let t10874 = t827 * t828 * t10872;
    let t10878 = t827 * t828 * t10666;
    let t10881 = t2741 * t2648;
    (t10863, t10867, t10870, t10871, t10872, t10874, t10878, t10881)
}
