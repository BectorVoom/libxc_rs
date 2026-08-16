//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 742/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk742(t2459: f64, t713: f64, t2574: f64, t265: f64, t766: f64, t729: f64, t762: f64, t1882: f64, t2528: f64, t760: f64, t255: f64, t2569: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10039 = t2459 * t713;
    let t10041 = t2574 * t265 * t10039;
    let t10044 = t2459 * t766;
    let t10046 = t729 * t762 * t10044;
    let t10048 = t1882 * t2528;
    let t10050 = t760 * t760;
    let t10051 = 1.0_f64 / t10050;
    let t10052 = t255 * t10051;
    let t10053 = t2569 * t766;
    (t10039, t10041, t10044, t10046, t10048, t10050, t10051, t10052, t10053)
}
