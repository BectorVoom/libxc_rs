//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta488(t25986: f64, t3994: f64, t2661: f64, t3970: f64, t7271: f64, t4014: f64, t4059: f64, t7264: f64, t2482: f64, t27: f64, t7262: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t25987, t25988, t25989, t25990, t25992, t25994, t25997) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1833(t25986, t3994, t2661, t3970, t7271, t4014, t4059, t7264, t2482, t27, t7262);
    (t25987, t25988, t25989, t25990, t25992, t25994, t25997)
}
