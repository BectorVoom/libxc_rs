//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta842 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2718;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta842(t12915: f64, t17344: f64, t20747: f64, t247: f64, t1261: f64, t44693: f64, t6421: f64, t12910: f64, t12916: f64, t20857: f64, t1208: f64, t21332: f64, t225: f64, t480: f64, t17289: f64, t1803: f64, t1222: f64, t6652: f64, t697: f64, t17709: f64, t20958: f64, t1235: f64, t371: f64, t6645: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70129, t70133, t70140, t70208) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2718(t12915, t17344, t20747, t247, t1261, t44693, t6421, t12910, t12916, t20857, t1208, t21332);
        let (t70209, t70210, t70221, t70225, t70250, t70263) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2719(t225, t70208, t480, t17289, t1803, t1222, t6652, t697, t12916, t17709, t20958, t1235, t371, t6645, t676);
    (t70129, t70133, t70140, t70208, t70209, t70210, t70221, t70225, t70250, t70263)
}
