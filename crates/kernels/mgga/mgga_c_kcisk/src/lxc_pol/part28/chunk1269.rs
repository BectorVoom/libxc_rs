//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1269/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1269<F: Float>(t110824: F, t110827: F, t110829: F, t110832: F, t110834: F, t110837: F, t110840: F, t110842: F, t110845: F, t110847: F, t110849: F, t110851: F, t110854: F, t110856: F, t31837: F, t3236: F) -> (F, F) {
    let t110858 = -0.485625e0 * t110824 + 0.809375e-1 * t110827 + 0.2428125e0 * t110829 + 0.485625e0 * t110832 + 0.2428125e0 * t110834 - 0.485625e0 * t110837 - 0.97125e0 * t110840 + 0.1125e1 * t110842 - 0.1875e0 * t110845 + 0.97125e0 * t110847 - 0.5625e0 * t110849 + 0.1125e1 * t110851 - 0.225e1 * t110854 + 0.225e1 * t110856;
    let t110859 = t3236 * t31837;
    (t110858, t110859)
}
