//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2346;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2347;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta538(t1263: f64, t5245: f64, t1122: f64, t1042: f64, t1234: f64, t5390: f64, t3704: f64, t5293: f64, t1121: f64, t1214: f64, t606: f64, t1250: f64, t17353: f64, t1802: f64, t3147: f64, t3597: f64, t3594: f64, t1244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17500, t17501, t17502, t17505) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2346(t1263, t5245, t1122, t1042, t1234, t5390);
        let (t17509, t17513, t17514, t17515, t17524, t17525, t17528) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2347(t3704, t5293, t1121, t1214, t606, t1250, t17353, t1802, t3147, t3597, t3594, t1244);
    (t17500, t17501, t17502, t17505, t17509, t17513, t17514, t17515, t17524, t17525, t17528)
}
