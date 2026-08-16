//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta114 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk645;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta114(t680: f64, t130: f64, t146: f64, t2566: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t2580, t2581, t2582, t2583, t2584, t2585, t2587) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk645(t680, t130, t146, t2566);
    (t2580, t2581, t2582, t2583, t2584, t2585, t2587)
}
