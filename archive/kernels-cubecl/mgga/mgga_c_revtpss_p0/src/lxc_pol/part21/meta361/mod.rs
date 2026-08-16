//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1721;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta361<F: Float>(t11940: F, t366: F, t11202: F, t373: F, t371: F, t372: F, t1053: F, t3204: F, t127: F, t3218: F, t1025: F, t1058: F, t3191: F) -> (F, F, F, F, F, F, F) {
        let (t11941, t11942, t11944, t11947, t11951, t11952, t11954) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1721::<F>(t11940, t366, t11202, t373, t371, t372, t1053, t3204, t127, t3218, t1025, t1058, t3191);
    (t11941, t11942, t11944, t11947, t11951, t11952, t11954)
}
