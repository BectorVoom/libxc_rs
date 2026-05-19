//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 312/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk312<F: Float>(t1888: F, t209: F, t469: F, t6: F, t1868: F, t1231: F, t1839: F, t490: F, t1835: F, t489: F, t1181: F, t1195: F, t1227: F, t1473: F, t1513: F, t1870: F, t1875: F, t467: F, t488: F) -> (F, F, F, F, F) {
    let t1891 = t469 * t6 * t1888 * t209;
    let t1895 = t469 * t1868 * t209;
    let t1900 = t1231 * t490 * t1839;
    let t1904 = t489 * t490 * t1835;
    let t1907 = F::cast_from(0.54879112805223954488e-1_f64) * t1181 * t1870 + F::cast_from(0.12805126321218922714e0_f64) * t1473 + F::cast_from(0.10975822561044790898e0_f64) * t1195 * t1875 - F::cast_from(0.27439556402611977244e-1_f64) * t467 * t1891 - F::cast_from(0.27439556402611977244e-1_f64) * t467 * t1895 + t1227 + F::cast_from(0.25610252642437845428e0_f64) * t1513 + F::cast_from(0.16463733841567186346e0_f64) * t488 * t1900 - F::cast_from(0.54879112805223954488e-1_f64) * t488 * t1904;
    (t1891, t1895, t1900, t1904, t1907)
}
