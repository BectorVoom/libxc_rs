//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 654/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk654(t3591: f64, t3592: f64, t3606: f64, t3607: f64, t153: f64, t156: f64, t1596: f64, t1601: f64, t1608: f64, t1611: f64, t168: f64, t1937: f64, t242: f64, t245: f64, t2520: f64, t2526: f64, t2531: f64, t2837: f64, t3373: f64, t3380: f64) -> (f64, f64) {
    let t3609 = t3591 + t3592 + t3606 + t3607;
    let t3617 = -t1596 + 0.16752564107100880375e0_f64 * t2520 + t1601 - 0.83762820535504401876e-1_f64 * t3380 * t242 - 0.16752564107100880375e0_f64 * t2526 - t1608 - t1611 + 0.39794582218349216586e-1_f64 * t2531 - 0.11938374665504764976e-1_f64 * t168 * t245 * t3609 + t1937 - 0.11389037339096724978e1_f64 * t2837 + 0.42708890021612718669e0_f64 * t153 * t156 * t3373;
    (t3609, t3617)
}
