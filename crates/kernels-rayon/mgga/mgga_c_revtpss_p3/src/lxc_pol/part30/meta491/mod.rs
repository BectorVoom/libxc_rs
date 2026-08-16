//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1840;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1841;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta491(t26034: f64, t545: f64, t2028: f64, t3920: f64, t7246: f64, t2023: f64, t2453: f64, t3908: f64, t2022: f64, t3923: f64, t543: f64, t7301: f64, t72: f64, t7307: f64, t686: f64, t7284: f64, t1426: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26035, t26036, t26040, t26041, t26043, t26044, t26046) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1840(t26034, t545, t2028, t3920, t7246, t2023, t2453, t3908, t2022, t3923, t543, t7301);
        let (t26049, t26050) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1841(t72, t7307, t686);
        let (t26051, t26053, t26054) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1842(t26050, t7284, t1426, t2023, t786);
    (t26035, t26036, t26040, t26041, t26043, t26044, t26046, t26049, t26050, t26051, t26053, t26054)
}
