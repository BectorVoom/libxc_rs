//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta111 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk656;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk657;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk658;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk659;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta111<F: Float>(t2629: F, t2630: F, t73: F, t853: F, t820: F, t843: F, t849: F, t857: F, t212: F, t27: F, t225: F, t816: F, t240: F, t823: F, t243: F, t836: F, t231: F) -> (F, F, F, F, F, F, F) {
        let (t2632, t2638, t2652) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk656::<F>(t2629, t2630, t73, t853, t820, t843, t849);
        let (t2653, t2661) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk657::<F>(t2652, t857, t212, t27, t225, t816);
        let t2662 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk658::<F>(t240, t823);
        let t2664 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk659::<F>(t243, t836, t231);
    (t2632, t2638, t2652, t2653, t2661, t2662, t2664)
}
