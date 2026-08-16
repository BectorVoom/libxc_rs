//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 310/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk310(t1867: f64, t6: f64, t1184: f64, t469: f64, t1475: f64, t589: f64, t221: f64, t1847: f64, t205: f64, t1205: f64, t1839: f64, t1835: f64, t472: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1868 = t6 * t1867;
    let t1870 = t469 * t1868 * t1184;
    let t1874 = t1475 * t589;
    let t1875 = t221 * t1874;
    let t1878 = t1847 * t205;
    let t1882 = t1205 * t1839;
    let t1885 = t472 * t1835;
    (t1868, t1870, t1875, t1878, t1882, t1885)
}
