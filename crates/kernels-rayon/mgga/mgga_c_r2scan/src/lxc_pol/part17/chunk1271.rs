//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1271/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1271(t39054: f64, t42170: f64, t43826: f64, t43829: f64, t43832: f64, t44051: f64, t44054: f64, t44057: f64, t44061: f64, t44064: f64, t44068: f64, t44072: f64, t44074: f64, t44077: f64, t44080: f64) -> f64 {
    let t44962 = t44051 + t44054 + t44057 - 0.72042316457491791901e-3_f64 * t43826 - 0.30487649791575028312e-3_f64 * t43829 + t44061 - t44064 + t44068 - 0.81300399444200075499e-3_f64 * t43832 + t44072 + t44074 - t39054 - t44077 - t44080 - t42170;
    t44962
}
