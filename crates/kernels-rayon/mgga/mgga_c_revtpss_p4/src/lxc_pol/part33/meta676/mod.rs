//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta676 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2208;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta676(t1937: f64, t85360: f64, t18245: f64, t6993: f64, t1448: f64, t30122: f64, t25082: f64, t28197: f64, t105886: f64, t1312: f64, t1936: f64, t75439: f64) -> (f64, f64, f64, f64, f64) {
        let (t109196, t109198, t109202, t109204, t109222) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2208(t1937, t85360, t18245, t6993, t1448, t30122, t25082, t28197, t105886, t1312, t1936, t75439);
    (t109196, t109198, t109202, t109204, t109222)
}
