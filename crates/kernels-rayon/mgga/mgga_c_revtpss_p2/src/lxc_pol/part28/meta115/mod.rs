//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk677;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta115(t158: f64, t2609: f64, t157: f64, t37: f64, t190: f64, t2251: f64, t606: f64, t750: f64, t706: f64, t186: f64, t215: f64, t685: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2610, t2611, t2612, t2614, t2615, t2616, t2617, t2619) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk677(t158, t2609, t157, t37, t190, t2251, t606, t750, t706, t186, t215, t685);
    (t2610, t2611, t2612, t2614, t2615, t2616, t2617, t2619)
}
