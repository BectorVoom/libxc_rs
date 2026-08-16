//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 595/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk595(t2373: f64, t4898: f64, t4854: f64, t837: f64, t2454: f64, t2461: f64, t3640: f64, t3687: f64, t4770: f64, t4774: f64, t4778: f64, t4787: f64, t4794: f64, t4800: f64, t4802: f64, t4806: f64, t4809: f64, t4812: f64) -> (f64, f64, f64) {
    let t4900 = 2.0_f64 * t2373 * t4898;
    let t4904 = t4854 * t837;
    let t4919 = -0.1294625e1_f64 * t4787 + 0.258925e1_f64 * t4794 + t2454 + 0.20128333333333333334e0_f64 * t3640 - 0.20128333333333333333e0_f64 * t4770 + 0.60385e0_f64 * t4774 - 0.301925e0_f64 * t4778 + 0.82524375e-1_f64 * t4800 + 0.16504875e0_f64 * t4802 + t2461 + 0.11038e0_f64 * t3687 - 0.27595e-1_f64 * t4806 + 0.16557e0_f64 * t4809 - 0.82785e-1_f64 * t4812;
    (t4900, t4904, t4919)
}
