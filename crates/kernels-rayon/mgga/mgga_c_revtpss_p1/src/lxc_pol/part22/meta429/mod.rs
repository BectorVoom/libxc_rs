//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2050;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2051;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta429(t14622: f64, t4401: f64, t2414: f64, t4311: f64, t10428: f64, t1522: f64, t10613: f64, t10592: f64, t10596: f64, t10604: f64, t10611: f64, t14442: f64, t14443: f64, t14444: f64, t14615: f64, t14618: f64, t14620: f64, t14621: f64, t9542: f64, t14609: f64, t14610: f64, t14612: f64, t225: f64, t73: f64, t830: f64, t1544: f64, t2475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14624, t14626, t14628, t14629, t14630) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2050(t14622, t4401, t2414, t4311, t10428, t1522, t10613, t10592, t10596, t10604, t10611, t14442, t14443, t14444, t14615, t14618, t14620, t14621, t9542);
        let (t14633, t14643, t14648) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2051(t14609, t14610, t14612, t14630, t225, t73, t830, t1544, t2475);
    (t14624, t14626, t14628, t14629, t14633, t14643, t14648)
}
