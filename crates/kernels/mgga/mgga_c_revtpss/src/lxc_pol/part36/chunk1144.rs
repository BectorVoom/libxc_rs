//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1144/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1144<F: Float>(t213: F, t30055: F, t689: F, t6896: F, t7242: F, t22399: F, t26054: F, t27888: F, t27899: F, t27884: F, t27873: F, t97700: F, t98041: F, t22453: F, t94901: F, t108368: F, t25895: F) -> (F, F, F, F, F, F, F, F, F) {
    let t108395 = t213 * t30055;
    let t108411 = t689 * t7242 * t6896;
    let t108422 = t26054 * t22399;
    let t108431 = t27899 * t27888;
    let t108435 = t27884 * t27888;
    let t108438 = t97700 * t27873;
    let t108440 = t98041 * t27873;
    let t108455 = t94901 * t22453;
    let t108464 = t25895 * t108368;
    (t108395, t108411, t108422, t108431, t108435, t108438, t108440, t108455, t108464)
}
