//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 943/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk943<F: Float>(t17541: F, t564: F, t2730: F, t5171: F, t1365: F, t670: F, t671: F, t1985: F, t666: F, t226: F, t5903: F, t230: F, t5907: F) -> (F, F, F, F, F, F) {
    let t17543 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t17541 * t564;
    let t17544 = t2730 * t5171;
    let t17545 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t17544;
    let t17548 = F::cast_from(0.22443641344164119597e0_f64) * t670 * t1365 * t671;
    let t17549 = t666 * t1985;
    let t17552 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t226 * t5903;
    let t17553 = t5907 * t230;
    (t17543, t17545, t17548, t17549, t17552, t17553)
}
