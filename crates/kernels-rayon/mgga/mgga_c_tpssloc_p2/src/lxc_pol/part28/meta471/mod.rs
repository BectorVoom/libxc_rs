//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1680;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1681;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta471(t23185: f64, t25245: f64, t234: f64, t6604: f64, t1484: f64, t252: f64, t776: f64, t25038: f64, t7528: f64, t794: f64, t6562: f64, t13380: f64, t232: f64, t6646: f64, t1888: f64, t6579: f64, t7525: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25246, t25248) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1680(t23185, t25245, t234, t6604);
        let t25249 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1681(t1484, t252);
        let (t25250, t25251, t25252, t25258, t25259, t25272, t25273, t25274, t25277) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1682(t25249, t776, t25248, t25038, t7528, t794, t6562, t13380, t232, t6646, t1888, t6579, t7525);
    (t25246, t25248, t25249, t25250, t25251, t25252, t25258, t25259, t25272, t25273, t25274, t25277)
}
