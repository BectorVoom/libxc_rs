//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1212/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1212<F: Float>(t39629: F, t39637: F, t39640: F, t39642: F, t37718: F, t37721: F, t39632: F, t39635: F, t39645: F, t39647: F, t39650: F, t41474: F) -> F {
    let t41475 = F::new(0.13869154784086829701e1) * t39629;
    let t41478 = F::new(0.32927245914677557993e-1) * t39637;
    let t41479 = F::new(0.65854491829355115984e-1) * t39640;
    let t41480 = F::new(0.11708928647259339622e0) * t39642;
    let t41484 = -F::new(0.95219938395347901946e-2) * t37718 - F::new(0.28565981518604370584e-1) * t37721 + t41474 + t41475 + F::new(0.52396431978519890152e-1) * t39632 - F::new(0.25426783770825854453e1) * t39635 - t41478 - t41479 + t41480 + F::new(0.52009330440325611378e0) * t39645 + F::new(0.32927245914677557992e0) * t39647 - F::new(0.13099107994629972538e-1) * t39650;
    t41484
}
