//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta598(t2028: f64, t27980: f64, t13790: f64, t4102: f64, t685: f64, t72: f64, t25875: f64, t1444: f64, t5740: f64, t675: f64, t94395: f64, t14109: f64, t25900: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t97676, t97680, t97682, t97685, t97687, t97688) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2028(t2028, t27980, t13790, t4102, t685, t72, t25875, t1444, t5740, t675, t94395, t14109, t25900);
    (t97676, t97680, t97682, t97685, t97687, t97688)
}
