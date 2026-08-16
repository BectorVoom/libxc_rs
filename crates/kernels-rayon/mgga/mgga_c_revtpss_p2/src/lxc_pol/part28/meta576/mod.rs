//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta576 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta576(t11752: f64, t7111: f64, t11755: f64, t11937: f64, t25500: f64, t1024: f64, t25553: f64, t25495: f64, t3215: f64, t11817: f64, t7117: f64, t3223: f64, t7125: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t93702, t93704, t93713, t93715, t93718, t93720, t93722) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2040(t11752, t7111, t11755, t11937, t25500, t1024, t25553, t25495, t3215, t11817, t7117, t3223, t7125);
    (t93702, t93704, t93713, t93715, t93718, t93720, t93722)
}
