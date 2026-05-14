//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1146/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1146<F: Float>(t28365: F, t8392: F, t1882: F, t28448: F, t28236: F, t28445: F, t6854: F, t8232: F, t28426: F, t28417: F, t681: F, t89: F, t28248: F, t6871: F, t28278: F, t28273: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t110803 = 4.0 / 9.0 * t8392 * t28365;
    let t110805 = 2.0 / 9.0 * t1882 * t28448;
    let t110817 = 4.0 / 9.0 * t1882 * t28236;
    let t110826 = 2.0 / 9.0 * t1882 * t28445;
    let t110845 = t8232 * t6854;
    let t110859 = 2.0 / 9.0 * t1882 * t28426;
    let t110872 = 2.0 / 9.0 * t89 * t681 * t28417;
    let t110889 = 2.0 / 9.0 * t1882 * t28248;
    let t110890 = t8232 * t6871;
    let t110931 = 2.0 / 9.0 * t1882 * t28278;
    let t110933 = 2.0 / 9.0 * t1882 * t28273;
    (t110803, t110805, t110817, t110826, t110845, t110859, t110872, t110889, t110890, t110931, t110933)
}
