//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 652/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk652<F: Float>(t1816: F, t5137: F, t639: F, t1702: F, t617: F, t1809: F, t1620: F, t661: F, t1815: F, t5038: F, t2677: F, t5029: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5138 = t5137 * t1816;
    let t5139 = t639 * t5138;
    let t5140 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t5139;
    let t5141 = t1702 * t617;
    let t5142 = t1809 * t5141;
    let t5144 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1620 * t5142;
    let t5145 = t1702 * t661;
    let t5146 = t1815 * t5145;
    let t5148 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t639 * t5146;
    let t5149 = t1809 * t5038;
    let t5151 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t639 * t5149;
    let t5152 = t2677 * t5029;
    (t5138, t5140, t5141, t5142, t5144, t5145, t5146, t5148, t5149, t5151, t5152)
}
