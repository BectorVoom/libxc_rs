//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta291 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1044;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta291<F: Float>(t760: F, t9318: F, t2251: F, t750: F, t2611: F, t2398: F, t2615: F, t2609: F, t717: F, t162: F, t9544: F, t158: F, t755: F, t9586: F, t2619: F, t2622: F, t2390: F, t72: F, t757: F, t2629: F, t9863: F, t123: F, t752: F, t2630: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10554, t10556, t10561, t10563, t10566) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1044::<F>(t760, t9318, t2251, t750, t2611, t2398, t2615, t2609, t717, t162, t9544, t158);
        let (t10568, t10569, t10574, t10577, t10579) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1045::<F>(t755, t9586, t2619, t2622, t2390, t72, t757, t2629, t9863, t123, t752, t2630);
    (t10554, t10556, t10561, t10563, t10566, t10568, t10569, t10574, t10577, t10579)
}
