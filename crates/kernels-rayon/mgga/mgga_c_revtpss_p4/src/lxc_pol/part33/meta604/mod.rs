//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta604(t26948: f64, t487: f64, t8945: f64, t26936: f64, t3736: f64, t7635: f64, t3566: f64, t1269: f64, t7642: f64, t13032: f64, t26848: f64, t12881: f64, t7624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t97041, t97050, t97065, t97066, t97082, t97129, t97141) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2028(t26948, t487, t8945, t26936, t3736, t7635, t3566, t1269, t7642, t13032, t26848, t12881, t7624);
    (t97041, t97050, t97065, t97066, t97082, t97129, t97141)
}
