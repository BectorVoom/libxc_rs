//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 312/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk312(t1888: f64, t209: f64, t469: f64, t6: f64, t1868: f64, t1231: f64, t1839: f64, t490: f64, t1835: f64, t489: f64, t1181: f64, t1195: f64, t1227: f64, t1473: f64, t1513: f64, t1870: f64, t1875: f64, t467: f64, t488: f64) -> (f64, f64, f64, f64, f64) {
    let t1891 = t469 * t6 * t1888 * t209;
    let t1895 = t469 * t1868 * t209;
    let t1900 = t1231 * t490 * t1839;
    let t1904 = t489 * t490 * t1835;
    let t1907 = 0.54879112805223954488e-1_f64 * t1181 * t1870 + 0.12805126321218922714e0_f64 * t1473 + 0.10975822561044790898e0_f64 * t1195 * t1875 - 0.27439556402611977244e-1_f64 * t467 * t1891 - 0.27439556402611977244e-1_f64 * t467 * t1895 + t1227 + 0.25610252642437845428e0_f64 * t1513 + 0.16463733841567186346e0_f64 * t488 * t1900 - 0.54879112805223954488e-1_f64 * t488 * t1904;
    (t1891, t1895, t1900, t1904, t1907)
}
