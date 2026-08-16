//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1090/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1090(t2410: f64, t344: f64, t148: f64, t931: f64, t179: f64, t404: f64, t824: f64, t2411: f64, t465: f64, t154: f64, t385: f64, t386: f64, t4932: f64) -> (f64, f64, f64, f64, f64) {
    let t19140 = 1.0_f64 / t2410 / t344;
    let t19150 = t148 * t931;
    let t19153 = t404 * t179 * t19150 * t824;
    let t19155 = t465 * t2411;
    let t19163 = 5.0_f64 / 486.0_f64 * t385 * t154 * t4932 * t386;
    (t19140, t19150, t19153, t19155, t19163)
}
