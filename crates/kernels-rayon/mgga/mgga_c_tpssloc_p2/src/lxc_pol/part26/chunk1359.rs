//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1359/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1359(t27561: f64, t7327: f64, t52537: f64, t7376: f64, t24826: f64, t24834: f64, t1209: f64, t85964: f64, t3032: f64, t475: f64, t3507: f64, t7348: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t86015 = t7327 * t27561;
    let t86016 = t52537 * t7376;
    let t86020 = t24826 * t24834;
    let t86022 = t85964 * t1209;
    let t86023 = t3032 * t475;
    let t86032 = t7348 * t3507;
    (t86015, t86016, t86020, t86022, t86023, t86032)
}
