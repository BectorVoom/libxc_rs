//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 592/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk592<F: Float>(t245: F, t18: F, t267: F, t1178: F, t1577: F, t21: F, t363: F, t4011: F, t5: F, t776: F, t920: F, t1217: F, t2648: F) -> (F, F, F) {
    let t246 = F::cast_from(10000000.0_f64) <= t245;
    let t4021 = t267 * t18;
    let t4026 = piecewise3::<F>(t246, F::cast_from(0.0_f64), t5 * t4011 * t21 / F::cast_from(4.0_f64) + t5 * t1178 * t363 / F::cast_from(4.0_f64) + t5 * t776 * t920 / F::cast_from(4.0_f64) + t5 * t4021 * t1577 / F::cast_from(2.0_f64));
    let t4027 = t2648 * t1217;
    (t4021, t4026, t4027)
}
