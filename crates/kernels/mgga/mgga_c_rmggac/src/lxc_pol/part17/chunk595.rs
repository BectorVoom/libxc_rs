//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 595/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk595<F: Float>(t7788: F, t8650: F, t7785: F, t8626: F, t7829: F, t8632: F, t7782: F, t8636: F, t2392: F, t321: F, t262: F, t333: F, t7835: F, t8622: F, t2068: F, t8709: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8893 = t7788 * t8650;
    let t8895 = t7785 * t8626;
    let t8897 = t7829 * t8632;
    let t8899 = t7782 * t8636;
    let t8901 = t2392 * t321;
    let t8902 = t262 * t8901;
    let t8903 = t7788 * t8902;
    let t8905 = t2392 * t333;
    let t8906 = t262 * t8905;
    let t8907 = t7782 * t8906;
    let t8909 = t7835 * t8622;
    let t8911 = t2068 * t8709;
    (t8893, t8895, t8897, t8899, t8901, t8902, t8903, t8905, t8906, t8907, t8909, t8911)
}
