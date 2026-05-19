//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 647/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk647<F: Float>(t1639: F, t56: F, t5064: F, t11: F, t5048: F, t625: F, t25: F, t5065: F, t5069: F, t5072: F, t5075: F, t5078: F, t5082: F, t5083: F, t5085: F, t5087: F) -> (F, F, F, F, F, F) {
    let t5089 = t56 * t1639;
    let t5090 = t5089 * t5064;
    let t5091 = t11 * t5090;
    let t5093 = t625 * t5048;
    let t5094 = t11 * t5093;
    let t5096 = -F::cast_from(0.29629629629629629629e-2_f64) * t25 * t5065 + F::cast_from(0.14396666666666666667e0_f64) * t5069 - F::cast_from(0.71983333333333333335e-1_f64) * t5072 - F::new(0.21595e0) * t5075 + F::new(0.21595e0) * t5078 - t5082 - F::cast_from(0.47988888888888888888e-1_f64) * t5083 + F::cast_from(0.35991666666666666666e-1_f64) * t5085 + F::cast_from(0.23994444444444444444e-1_f64) * t5087 - F::cast_from(0.39990740740740740742e-1_f64) * t5091 - F::cast_from(0.35991666666666666667e-1_f64) * t5094;
    (t5089, t5090, t5091, t5093, t5094, t5096)
}
