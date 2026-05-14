//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 630/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk630<F: Float>(t1809: F, t5141: F, t1620: F, t1702: F, t661: F, t1815: F, t639: F, t5038: F, t2677: F, t5029: F, t1692: F, t617: F, t1726: F, t633: F, t4359: F, t220: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5142 = t1809 * t5141;
    let t5144 = 8.0 / 15.0 * t1620 * t5142;
    let t5145 = t1702 * t661;
    let t5146 = t1815 * t5145;
    let t5148 = 4.0 / 15.0 * t639 * t5146;
    let t5149 = t1809 * t5038;
    let t5151 = 8.0 / 15.0 * t639 * t5149;
    let t5152 = t2677 * t5029;
    let t5154 = 4.0 / 9.0 * t639 * t5152;
    let t5155 = t1692 * t617;
    let t5156 = t2677 * t5155;
    let t5158 = 8.0 / 9.0 * t1620 * t5156;
    let t5160 = 2.0 / 5.0 * t633 * t1726;
    let t5162 = -3.0 * t4359;
    let t5163 = t220 * t5162;
    (t5142, t5144, t5145, t5146, t5148, t5149, t5151, t5152, t5154, t5155, t5156, t5158, t5160, t5162, t5163)
}
