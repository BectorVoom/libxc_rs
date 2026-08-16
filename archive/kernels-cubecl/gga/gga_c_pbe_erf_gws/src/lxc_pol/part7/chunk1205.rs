//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1205/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1205<F: Float>(t5: F, t6439: F, t343: F, t2121: F, t337: F, t2134: F, t2074: F, t2122: F, t2147: F, t2120: F, t20270: F, t2276: F) -> (F, F, F, F) {
    let t21419 = t5 * t6439;
    let t21420 = t21419 * t343;
    let t21422 = t2121 * t337 * t21420;
    let t21424 = t2134 * t21422 / F::cast_from(24.0_f64);
    let t21427 = t2147 * t337 * t2122 * t2074;
    let t21429 = t2120 * t21427 / F::cast_from(8.0_f64);
    let t21430 = t2276 * t20270;
    (t21419, t21424, t21429, t21430)
}
