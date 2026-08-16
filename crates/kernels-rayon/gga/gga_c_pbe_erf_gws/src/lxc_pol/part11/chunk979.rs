//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 979/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk979(t1672: f64, t185: f64, t3444: f64, t2612: f64, t7459: f64, t10033: f64, t164: f64, t163: f64, t169: f64, t3569: f64, t784: f64, t3379: f64, t551: f64, t553: f64, t837: f64) -> (f64, f64, f64, f64, f64) {
    let t33281 = t185 * t1672 * t3444;
    let t33298 = t2612 * t7459;
    let t33381 = t10033 * t164;
    let t33385 = t169 * t784 * t3569 * t163;
    let t33389 = t837 * t3379 * t551 * t553;
    (t33281, t33298, t33381, t33385, t33389)
}
