//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 542/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk542(t5: f64, t1437: f64, t2235: f64, t2240: f64, t3951: f64, t3953: f64, t3958: f64, t4021: f64, t605: f64, t645: f64, t86: f64, t112: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t4025 = piecewise3(t8, 0.0_f64, -4.0_f64 * t1437 * t2235 + 20.0_f64 * t2240 * t3958 + t3951 * t86 - 4.0_f64 * t3953 * t645 - 4.0_f64 * t4021 * t605);
    let t4026 = t4025 * t112;
    (t4025, t4026)
}
