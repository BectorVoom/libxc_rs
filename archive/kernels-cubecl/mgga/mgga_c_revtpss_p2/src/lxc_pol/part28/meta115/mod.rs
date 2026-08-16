//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta115<F: Float>(t158: F, t2609: F, t157: F, t37: F, t190: F, t2251: F, t606: F, t750: F, t706: F, t186: F, t215: F, t685: F) -> (F, F, F, F, F, F, F, F) {
        let (t2610, t2611, t2612, t2614, t2615, t2616, t2617, t2619) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk677::<F>(t158, t2609, t157, t37, t190, t2251, t606, t750, t706, t186, t215, t685);
    (t2610, t2611, t2612, t2614, t2615, t2616, t2617, t2619)
}
