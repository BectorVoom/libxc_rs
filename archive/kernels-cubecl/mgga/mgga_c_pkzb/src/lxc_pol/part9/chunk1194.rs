//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1194/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1194<F: Float>(t1878: F, t218: F, t2774: F, t2778: F, t1079: F, t5555: F, t17351: F, t17354: F, t17357: F, t17403: F, t17405: F, t17408: F, t17411: F, t17414: F, t17417: F, t17454: F, t20705: F, t20708: F, t20710: F, t20717: F, t20719: F, t20745: F) -> (F, F, F, F) {
    let t20748 = t218 * t1878 * t2774;
    let t20749 = F::cast_from(0.82785e0_f64) * t20748;
    let t20751 = t218 * t1878 * t2778;
    let t20752 = F::cast_from(0.82785e0_f64) * t20751;
    let t20754 = t218 * t5555 * t1079;
    let t20757 = -F::cast_from(0.22076e1_f64) * t17405 + F::cast_from(0.82785e0_f64) * t17411 - F::cast_from(0.49671e0_f64) * t17414 - F::cast_from(0.16557e0_f64) * t17417 - F::cast_from(0.93932222222222222223e0_f64) * t20705 + F::cast_from(0.58258125e1_f64) * t20708 - F::cast_from(0.1237865625e0_f64) * t20710 + t17454 - F::cast_from(0.28179666666666666667e1_f64) * t17351 + F::cast_from(0.12077e1_f64) * t17354 - F::cast_from(0.301925e0_f64) * t17357 + t20717 - F::cast_from(0.905775e0_f64) * t20719 + F::cast_from(0.905775e0_f64) * t20745 + t20749 + t20752 - F::cast_from(0.73586666666666666667e0_f64) * t20754 + t17403 + F::cast_from(0.82785e0_f64) * t17408;
    (t20748, t20751, t20754, t20757)
}
