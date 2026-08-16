//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2077;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta600(t10073: f64, t25920: f64, t25938: f64, t25898: f64, t94889: f64, t25901: f64, t10115: f64, t2024: f64, t112: f64, t843: f64, t239: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t94919, t94921, t94922, t94931, t94974, t94975) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2077(t10073, t25920, t25938, t25898, t94889, t25901, t10115, t2024, t112, t843, t239, t655);
    (t94919, t94921, t94922, t94931, t94974, t94975)
}
