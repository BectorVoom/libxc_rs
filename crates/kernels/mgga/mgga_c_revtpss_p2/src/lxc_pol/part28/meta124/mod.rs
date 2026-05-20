//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta124 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk709;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk710;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk711;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta124<F: Float>(t2770: F, t2771: F, t2435: F, t871: F, t225: F, t785: F, t870: F, t2439: F, t123: F, t212: F, t676: F, t822: F) -> (F, F, F, F, F, F, F) {
        let t2772 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk709::<F>(t2770, t2771);
        let (t2776, t2777) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk710::<F>(t2435, t871, t225, t785);
        let (t2778, t2780, t2782) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk711::<F>(t2777, t870, t2439, t123, t212, t676);
        let t2783 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk712::<F>(t225, t822);
    (t2772, t2776, t2777, t2778, t2780, t2782, t2783)
}
