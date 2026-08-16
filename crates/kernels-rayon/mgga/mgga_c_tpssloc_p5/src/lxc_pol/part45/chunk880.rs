//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 880/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk880(t25373: f64, t31448: f64, t1914: f64, t606: f64, t1877: f64, t193: f64, t202: f64, t24339: f64, t24344: f64, t2522: f64, t31429: f64, t31434: f64, t31441: f64, t6665: f64, t7114: f64, t776: f64, t8566: f64, t868: f64, t870: f64) -> (f64, f64, f64) {
    let t31449 = t25373 * t31448;
    let t31451 = t606 * t1914;
    let t31477 = t193 * t202 * t31429 * t870 - t1877 * t1914 * t24339 + 2.0_f64 * t1877 * t24344 * t31448 - t1877 * t31434 * t868 - t1877 * t6665 * t7114 - 3.0_f64 * t2522 * t31441 * t7114 + 3.0_f64 * t2522 * t776 * t8566;
    (t31449, t31451, t31477)
}
