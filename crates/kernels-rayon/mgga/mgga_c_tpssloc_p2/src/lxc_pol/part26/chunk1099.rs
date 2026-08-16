//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1099/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1099(t22845: f64, t22847: f64, t3872: f64, t6952: f64, t281: f64, t6931: f64, t1351: f64, t22705: f64, t236: f64, t550: f64, t2003: f64, t3862: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22848 = t22845 * t22847;
    let t22850 = t6952 * t3872;
    let t22852 = t6931 * t281;
    let t22855 = t22705 * t236 * t1351 * t550;
    let t22856 = t22852 * t22855;
    let t22858 = t2003 * t3862;
    (t22848, t22850, t22852, t22855, t22856, t22858)
}
