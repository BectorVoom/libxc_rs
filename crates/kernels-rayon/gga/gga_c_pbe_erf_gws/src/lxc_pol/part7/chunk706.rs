//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 706/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk706(t1552: f64, t19: f64, t506: f64, t299: f64, t481: f64, t799: f64, t119: f64, t1533: f64, t155: f64, t1557: f64, t1513: f64, t4516: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5761 = t1552 * t506 * t19;
    let t5763 = t799 * t299 * t481;
    let t5764 = t5761 * t5763;
    let t5767 = t119 * t155 * t1533;
    let t5768 = t1557 * t5767;
    let t5770 = t1513 * t5767;
    let t5771 = 0.14615125e1_f64 * t5770;
    let t5772 = param_hyb_omega_0 * t4516;
    (t5761, t5763, t5764, t5768, t5771, t5772)
}
