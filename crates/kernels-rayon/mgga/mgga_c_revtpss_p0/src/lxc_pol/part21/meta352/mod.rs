//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta352(t11735: f64, t345: f64, t10345: f64, t344: f64, t247: f64, t2858: f64, t3109: f64, t1063: f64, t1066: f64, t11160: f64, t1068: f64, t11707: f64, t11712: f64, t11714: f64, t11723: f64, t11728: f64, t11730: f64, t11732: f64, t3091: f64, t3101: f64, t3106: f64, t3177: f64, t3184: f64, t348: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t11737, t11738, t11744, t11745, t11748, t11751) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1693(t11735, t345, t10345, t344, t247, t2858, t3109, t1063, t1066, t11160, t1068, t11707, t11712, t11714, t11723, t11728, t11730, t11732, t3091, t3101, t3106, t3177, t3184, t348);
    (t11737, t11738, t11744, t11745, t11748, t11751)
}
