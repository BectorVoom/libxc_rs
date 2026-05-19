//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 928/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk928<F: Float>(t245: F, t18: F, t776: F, t1178: F, t14366: F, t1577: F, t1580: F, t21: F, t2624: F, t267: F, t363: F, t4011: F, t4021: F, t5: F, t7742: F, t920: F) -> F {
    let t246 = F::cast_from(10000000.0_f64) <= t245;
    let t14379 = t776 * t18;
    let t14389 = piecewise3::<F>(t246, F::new(0.0), t5 * t14366 * t21 / F::new(4.0) + t5 * t4011 * t363 / F::new(2.0) + t5 * t1178 * t1580 / F::new(4.0) + t5 * t2624 * t920 / F::new(4.0) + t5 * t14379 * t1577 + t5 * t267 * t1577 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t5 * t4021 * t7742);
    t14389
}
