//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 542/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk542<F: Float>(t437: F, t7733: F, t1537: F, t1760: F, t360: F, t18: F, t1577: F) -> (F, F, F, F, F) {
    let t7734 = t7733 * t437;
    let t7736 = t1537 * t1760;
    let t7741 = t360 * t360;
    let t7742 = F::cast_from(1.0_f64) / t7741;
    let t7743 = t18 * t7742;
    let t7745 = F::cast_from(6.0_f64) * t1577 - F::cast_from(6.0_f64) * t7743;
    (t7734, t7736, t7741, t7742, t7745)
}
