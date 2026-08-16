//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 714/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk714<F: Float>(t3649: F, t462: F, t2633: F, t2642: F, t2644: F, t2651: F, t2678: F, t3452: F, t3613: F, t3617: F, t3636: F, t3640: F, t3643: F, t3645: F, t3647: F, t493: F) -> (F, F) {
    let t3650 = t462 * t3649;
    let t3651 = -t3613 - t3452 - F::cast_from(0.5848223622634646207e0_f64) * t3617 + F::cast_from(0.19751673498613801407e-1_f64) * t3636 * t493 + t2633 - F::cast_from(0.18311447306006545054e-3_f64) * t3640 - t2642 - F::cast_from(0.5848223622634646207e0_f64) * t2644 + t2651 - t2678 - F::cast_from(4.0_f64) * t3643 + F::cast_from(4.0_f64) * t3645 + t462 * t3647 + t3650;
    (t3650, t3651)
}
