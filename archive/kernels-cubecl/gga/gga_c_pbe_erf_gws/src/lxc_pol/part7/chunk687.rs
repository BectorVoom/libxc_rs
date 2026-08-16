//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 687/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk687<F: Float>(t5522: F, t5523: F, t639: F, t1824: F, t5312: F, t1769: F, t610: F, t1827: F, t587: F, t1821: F, t4972: F, t2559: F, t4963: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5524 = t5522 * t5523;
    let t5526 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t639 * t5524;
    let t5528 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t5312 * t1824;
    let t5529 = t1769 * t610;
    let t5530 = t1827 * t5529;
    let t5532 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t587 * t5530;
    let t5533 = t1821 * t4972;
    let t5535 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t587 * t5533;
    let t5536 = t2559 * t4963;
    (t5524, t5526, t5528, t5529, t5530, t5532, t5533, t5535, t5536)
}
