//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta464(t11108: f64, t1699: f64, t3022: f64, t4725: f64, t11465: f64, t1633: f64, t3015: f64, t981: f64, t3026: f64, t4719: f64, t1695: f64, t3075: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t15566, t15571, t15573, t15575, t15577, t15578) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2145(t11108, t1699, t3022, t4725, t11465, t1633, t3015, t981, t3026, t4719, t1695, t3075);
    (t15566, t15571, t15573, t15575, t15577, t15578)
}
