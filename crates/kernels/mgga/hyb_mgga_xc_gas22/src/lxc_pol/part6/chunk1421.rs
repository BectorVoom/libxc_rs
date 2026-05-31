//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1421/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1421<F: Float>(t1161: F, t2893: F, t4512: F, t1117: F, t11279: F, t13638: F, t26093: F, t26096: F, t26345: F, t2821: F, t2834: F, t2838: F, t30703: F, t30723: F, t30733: F, t30736: F, t30739: F, t30748: F, t3661: F, t3663: F, t7643: F, t9440: F, t9523: F, t9535: F, t9703: F) -> (F, F) {
    let t30752 = t1161 * t4512 * t2893;
    let t30757 = -F::cast_from(3200.0_f64) / F::cast_from(81.0_f64) * t3661 * t30723 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t7643 * t30703 + F::cast_from(800.0_f64) / F::cast_from(27.0_f64) * t9703 * t11279 + F::cast_from(4000.0_f64) * t26096 * t3663 * t13638 - F::cast_from(4000.0_f64) * t30733 * t9523 - F::cast_from(5600.0_f64) * t26093 * t30736 + F::cast_from(5600.0_f64) * t30739 * t9535 - F::cast_from(800.0_f64) / F::cast_from(3.0_f64) * t26345 * t30736 + F::cast_from(800.0_f64) / F::cast_from(3.0_f64) * t1117 * t9440 * t9535 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2834 * t30748 - F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2838 * t30752 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t2821 * t30748;
    (t30752, t30757)
}
