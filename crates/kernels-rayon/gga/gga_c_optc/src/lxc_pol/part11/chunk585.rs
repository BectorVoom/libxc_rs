//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 585/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk585(t4793: f64, t780: f64, t2399: f64, t4786: f64, t787: f64, t2284: f64, t4768: f64, t25: f64, t4772: f64, t794: f64, t4776: f64, t2394: f64, t2404: f64, t3640: f64, t3687: f64, t4770: f64, t4774: f64, t4778: f64, t4787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4794 = t780 * t4793;
    let t4800 = t2399 * t4786;
    let t4802 = t787 * t4793;
    let t4805 = t2284 * t4768;
    let t4806 = t25 * t4805;
    let t4808 = t794 * t4772;
    let t4809 = t25 * t4808;
    let t4811 = t794 * t4776;
    let t4812 = t25 * t4811;
    let t4814 = -0.9494625e0_f64 * t4787 + 0.1898925e1_f64 * t4794 + t2394 + 0.19931111111111111111e0_f64 * t3640 - 0.19931111111111111111e0_f64 * t4770 + 0.59793333333333333334e0_f64 * t4774 - 0.29896666666666666667e0_f64 * t4778 + 0.15358125e0_f64 * t4800 + 0.3071625e0_f64 * t4802 + t2404 + 0.10954222222222222222e0_f64 * t3687 - 0.27385555555555555556e-1_f64 * t4806 + 0.16431333333333333333e0_f64 * t4809 - 0.82156666666666666667e-1_f64 * t4812;
    (t4794, t4800, t4802, t4805, t4806, t4808, t4809, t4811, t4812, t4814)
}
