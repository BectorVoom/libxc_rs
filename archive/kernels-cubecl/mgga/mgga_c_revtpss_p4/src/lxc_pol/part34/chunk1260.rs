//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1260/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1260<F: Float>(t108379: F, t7286: F, t27989: F, t97802: F, t213: F, t30055: F, t689: F, t6896: F, t7242: F, t22399: F, t26054: F, t27888: F, t27899: F) -> (F, F, F, F, F, F) {
    let t108380 = t108379 * t7286;
    let t108389 = t97802 * t27989;
    let t108395 = t213 * t30055;
    let t108411 = t689 * t7242 * t6896;
    let t108422 = t26054 * t22399;
    let t108431 = t27899 * t27888;
    (t108380, t108389, t108395, t108411, t108422, t108431)
}
