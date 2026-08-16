//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1279/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1279(t39091: f64, t39092: f64, t39093: f64, t39094: f64, t40513: f64, t43902: f64, t43907: f64, t44893: f64, t44897: f64, t44899: f64, t44902: f64, t44904: f64, t44907: f64, t44909: f64, t44912: f64) -> f64 {
    let t45011 = t44893 + 0.1440846329149835838e-2_f64 * t43902 + 0.72042316457491791901e-3_f64 * t43907 + t44897 + t44899 - t44902 - t44904 - t44907 - t44909 + t39091 - t39092 + t39093 + t44912 - t39094 - 0.60975299583150056624e-3_f64 * t40513;
    t45011
}
