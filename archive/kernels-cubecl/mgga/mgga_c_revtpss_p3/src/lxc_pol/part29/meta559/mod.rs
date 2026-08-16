//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1901;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta559<F: Float>(t13821: F, t27940: F, t13928: F, t26028: F, t241: F, t820: F, t94491: F, t13807: F, t13817: F, t13991: F, t13793: F, t13786: F) -> (F, F, F, F, F, F, F) {
        let (t98110, t98112, t98116, t98118, t98120, t98122, t98124) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1901::<F>(t13821, t27940, t13928, t26028, t241, t820, t94491, t13807, t13817, t13991, t13793, t13786);
    (t98110, t98112, t98116, t98118, t98120, t98122, t98124)
}
