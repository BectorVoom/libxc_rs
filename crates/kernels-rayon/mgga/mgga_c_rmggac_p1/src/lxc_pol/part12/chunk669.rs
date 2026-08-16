//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 669/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk669(t7785: f64, t8646: f64, t7788: f64, t8650: f64, t8626: f64, t7829: f64, t8632: f64, t7782: f64, t8636: f64, t2392: f64, t321: f64, t262: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8891 = t7785 * t8646;
    let t8893 = t7788 * t8650;
    let t8895 = t7785 * t8626;
    let t8897 = t7829 * t8632;
    let t8899 = t7782 * t8636;
    let t8901 = t2392 * t321;
    let t8902 = t262 * t8901;
    (t8891, t8893, t8895, t8897, t8899, t8901, t8902)
}
