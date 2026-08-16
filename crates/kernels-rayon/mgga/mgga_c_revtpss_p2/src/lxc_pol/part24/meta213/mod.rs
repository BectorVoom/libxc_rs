//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta213 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta213(t11249: f64, t3154: f64, t1036: f64, t11244: f64, t11240: f64, t357: f64, t246: f64, t676: f64) -> (f64, f64, f64, f64, f64) {
        let (t11250, t11255, t11256, t11257, t11262) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk957(t11249, t3154, t1036, t11244, t11240, t357, t246, t676);
    (t11250, t11255, t11256, t11257, t11262)
}
