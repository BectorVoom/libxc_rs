//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 585/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk585<F: Float>(t4793: F, t780: F, t2399: F, t4786: F, t787: F, t2284: F, t4768: F, t25: F, t4772: F, t794: F, t4776: F, t2394: F, t2404: F, t3640: F, t3687: F, t4770: F, t4774: F, t4778: F, t4787: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4794 = t780 * t4793;
    let t4800 = t2399 * t4786;
    let t4802 = t787 * t4793;
    let t4805 = t2284 * t4768;
    let t4806 = t25 * t4805;
    let t4808 = t794 * t4772;
    let t4809 = t25 * t4808;
    let t4811 = t794 * t4776;
    let t4812 = t25 * t4811;
    let t4814 = -F::new(0.9494625e0) * t4787 + F::new(0.1898925e1) * t4794 + t2394 + F::cast_from(0.19931111111111111111e0_f64) * t3640 - F::cast_from(0.19931111111111111111e0_f64) * t4770 + F::cast_from(0.59793333333333333334e0_f64) * t4774 - F::cast_from(0.29896666666666666667e0_f64) * t4778 + F::new(0.15358125e0) * t4800 + F::new(0.3071625e0) * t4802 + t2404 + F::cast_from(0.10954222222222222222e0_f64) * t3687 - F::cast_from(0.27385555555555555556e-1_f64) * t4806 + F::cast_from(0.16431333333333333333e0_f64) * t4809 - F::cast_from(0.82156666666666666667e-1_f64) * t4812;
    (t4794, t4800, t4802, t4805, t4806, t4808, t4809, t4811, t4812, t4814)
}
