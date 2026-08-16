//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 530/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk530<F: Float>(t15575: F, t2993: F, t1528: F, t18: F, t342: F, t4410: F, t630: F, t3103: F, t72: F, t11280: F, t1526: F, t1527: F, t15562: F, t15567: F, t15569: F, t2976: F, t2988: F, t3009: F, t3109: F, t343: F, t7704: F, t7707: F, t7710: F) -> F {
    let t15576 = t15575 * t2993;
    let t15579 = t1528 * t18;
    let t15584 = t342 * t630 * t4410;
    let t15589 = t72 * t3103;
    let t15593 = t2976 + t3109 + t7704 - t7707 / F::cast_from(36.0_f64) - t7710 / F::cast_from(12.0_f64) - t15562 / F::cast_from(36.0_f64) - t15567 * t15569 / F::cast_from(9.0_f64) - t1526 * t1527 * t2988 / F::cast_from(12.0_f64) + t15567 * t15576 / F::cast_from(6.0_f64) - t1526 * t11280 * t15579 / F::cast_from(6.0_f64) - t15584 / F::cast_from(12.0_f64) - t1526 * t1527 * t3009 / F::cast_from(12.0_f64) - t342 * t343 * t15589 / F::cast_from(4.0_f64);
    t15593
}
