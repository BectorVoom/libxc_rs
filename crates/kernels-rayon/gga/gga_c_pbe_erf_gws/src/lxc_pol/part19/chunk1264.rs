//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1264/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1264(t14895: f64, t8801: f64, t14188: f64, t26958: f64, t353: f64, t4228: f64, t814: f64, t859: f64, t14888: f64, t19906: f64, t15034: f64, t892: f64) -> (f64, f64, f64, f64, f64) {
    let t55672 = 7.0_f64 / 24.0_f64 * t8801 * t14895;
    let t55695 = 7.0_f64 / 72.0_f64 * t26958 * t14188;
    let t55698 = t859 * t353 * t4228 * t814;
    let t55702 = 7.0_f64 / 72.0_f64 * t19906 * t14888;
    let t55717 = t859 * t892 * t15034;
    (t55672, t55695, t55698, t55702, t55717)
}
