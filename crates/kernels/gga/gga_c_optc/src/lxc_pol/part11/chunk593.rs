//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 593/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk593<F: Float>(t4868: F, t818: F, t2500: F, t2507: F, t3640: F, t3687: F, t4770: F, t4774: F, t4778: F, t4787: F, t4794: F, t4800: F, t4802: F, t4806: F, t4809: F, t4812: F) -> (F, F) {
    let t4869 = t4868 * t818;
    let t4884 = -F::new(0.17648625e1) * t4787 + F::new(0.3529725e1) * t4794 + t2500 + F::cast_from(0.34431666666666666666e0_f64) * t3640 - F::cast_from(0.34431666666666666667e0_f64) * t4770 + F::new(0.103295e1) * t4774 - F::new(0.516475e0) * t4778 + F::new(0.31558125e0) * t4800 + F::new(0.6311625e0) * t4802 + t2507 + F::cast_from(0.13892666666666666667e0_f64) * t3687 - F::cast_from(0.34731666666666666667e-1_f64) * t4806 + F::new(0.20839e0) * t4809 - F::new(0.104195e0) * t4812;
    (t4869, t4884)
}
