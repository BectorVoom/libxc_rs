//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2034;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2035;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta608(t1294: f64, t21471: f64, t26921: f64, t7648: f64, t12627: f64, t7635: f64, t12587: f64, t7669: f64, t2155: f64, t44126: f64, t2028: f64, t27980: f64, t13790: f64, t4102: f64, t685: f64, t72: f64, t25875: f64, t1444: f64, t5740: f64, t675: f64, t94395: f64, t14109: f64, t25900: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97398, t97422, t97475, t97491, t97498, t97676) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2034(t1294, t21471, t26921, t7648, t12627, t7635, t12587, t7669, t2155, t44126, t2028, t27980);
        let (t97680, t97682, t97685, t97687, t97688) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2035(t13790, t4102, t685, t72, t25875, t97676, t1444, t5740, t675, t94395, t14109, t25900);
    (t97398, t97422, t97475, t97491, t97498, t97676, t97680, t97682, t97685, t97687, t97688)
}
