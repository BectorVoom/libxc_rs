//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1075;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1076;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta307<F: Float>(t3145: F, t334: F, t368: F, t3153: F, t73: F, t246: F, t676: F, t1046: F, t1041: F, t1038: F, t3229: F, t1036: F, t1033: F, t3169: F, t3173: F, t2866: F, t914: F, t2923: F, t910: F, t287: F, t2922: F, t275: F, t11132: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11243, t11249, t11262, t11264, t11267) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1075::<F>(t3145, t334, t368, t3153, t73, t246, t676, t1046, t1041, t1038, t3229, t1036);
        let (t11268, t11271, t11289, t11294, t11299, t11304) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1076::<F>(t1033, t11267, t3169, t3173, t2866, t914, t2923, t910, t287, t2922, t275, t11132);
    (t11243, t11249, t11262, t11264, t11268, t11271, t11289, t11294, t11299, t11304)
}
