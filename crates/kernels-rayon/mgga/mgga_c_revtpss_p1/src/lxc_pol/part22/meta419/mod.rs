//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta419 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2027;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta419(t57: f64, t13312: f64, t14413: f64, t14416: f64, t2251: f64, t2258: f64, t4384: f64, t606: f64, t81: f64, t14412: f64, t162: f64, t187: f64, t2615: f64, t4311: f64, zeta_threshold: f64, t10588: f64, t10577: f64, t10582: f64, t10584: f64, t10586: f64, t10592: f64, t11084: f64, t14385: f64, t14388: f64, t14392: f64, t14396: f64, t14397: f64, t1544: f64, t1940: f64, t2394: f64, t2403: f64, t4541: f64, t4546: f64, t890: f64, t9514: f64, t9517: f64, t9521: f64, t9524: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t14425, t14426, t14428, t14433) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2027(t57, t13312, t14413, t14416, t2251, t2258, t4384, t606, t81, t14412, t162, t187, t2615, t4311, zeta_threshold);
        let (t14434, t14435) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2028(t10588, t10577, t10582, t10584, t10586, t10592, t11084, t14385, t14388, t14392, t14396, t14397, t14428, t14433, t1544, t1940, t2394, t2403, t4541, t4546, t890, t9514, t9517, t9521, t9524);
    (t14425, t14426, t14428, t14433, t14434, t14435)
}
