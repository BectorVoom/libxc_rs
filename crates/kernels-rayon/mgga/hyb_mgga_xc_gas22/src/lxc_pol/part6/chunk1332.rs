//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1332/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1332(t24556: f64, t24559: f64, t24562: f64, t24658: f64, t24661: f64, t24664: f64, t24667: f64, t24670: f64, t24673: f64, t28907: f64, t28917: f64, t28919: f64) -> f64 {
    let t29023 = 0.5696775e1_f64 * t28907 + 0.3071625e0_f64 * t28917 + 0.1898925e1_f64 * t28919 - 0.1860237037037037037e1_f64 * t24556 + 0.15944888888888888889e1_f64 * t24559 - 0.59793333333333333334e0_f64 * t24562 + 0.10954222222222222222e1_f64 * t24658 + 0.10954222222222222222e1_f64 * t24661 - 0.14605629629629629629e1_f64 * t24664 - 0.32862666666666666666e0_f64 * t24667 - 0.65725333333333333332e0_f64 * t24670 - 0.32862666666666666666e0_f64 * t24673;
    t29023
}
