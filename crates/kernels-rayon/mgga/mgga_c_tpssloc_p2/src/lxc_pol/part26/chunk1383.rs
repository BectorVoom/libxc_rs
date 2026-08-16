//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1383/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1383(t24977: f64, t576: f64, t1395: f64, t7426: f64, t12513: f64, t12537: f64, t1396: f64, t1398: f64, t1404: f64, t2170: f64, t2174: f64, t24955: f64, t3: f64, t3932: f64, t3946: f64, t580: f64, t7416: f64, t85403: f64, t85405: f64, t85407: f64, t85412: f64, t86550: f64, t86553: f64) -> f64 {
    let t86557 = t576 * t24977;
    let t86559 = t1395 * t7426;
    let tv4rho3sigma2 = t3 * t580 * t86550 + t12513 * t2174 + t12537 * t2170 + 3.0_f64 * t1396 * t24977 + t1398 * t86553 + 3.0_f64 * t1404 * t24955 + 3.0_f64 * t3932 * t7426 + 3.0_f64 * t3946 * t7416 + 3.0_f64 * t85403 + 6.0_f64 * t85405 + 3.0_f64 * t85407 + 3.0_f64 * t85412 + 3.0_f64 * t86557 + 6.0_f64 * t86559;
    tv4rho3sigma2
}
