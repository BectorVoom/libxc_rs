//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta217 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk963;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk964;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta217(t11132: f64, t1034: f64, t360: f64, t11244: f64, t11240: f64, t3154: f64, t357: f64, t11249: f64, t905: f64, t3182: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11534, t11560, t11574, t11626, t11627, t11628, t11629, t11630, t11631) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk963(t11132, t1034, t360, t11244, t11240, t3154, t357);
        let (t11632, t11660, t11703) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk964(t11249, t11631, t3154, t905, t3182, t828);
    (t11534, t11560, t11574, t11626, t11627, t11628, t11629, t11630, t11631, t11632, t11660, t11703)
}
