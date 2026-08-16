//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1728/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1728(t1877: f64, t2057: f64, t2219: f64, t1408: f64, t24191: f64, t24339: f64, t25: f64, t25015: f64, t25021: f64, t25024: f64, t25028: f64, t2522: f64, t25366: f64, t25375: f64, t25377: f64, t25381: f64, t25385: f64, t25392: f64, t26563: f64, t26740: f64, t26744: f64, t26756: f64, t606: f64, t6542: f64, t6671: f64, t7110: f64, t7114: f64, t7475: f64, t7545: f64, t7845: f64) -> (f64, f64) {
    let t26774 = t1877 * t2057 * t2219;
    let t26775 = 3.0_f64 * t26563 * t25015 + 3.0_f64 / 2.0_f64 * t2522 * t7110 * t7475 - 3.0_f64 / 2.0_f64 * t24191 * t25021 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t25024 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t25028 + 3.0_f64 / 2.0_f64 * t2522 * t7845 * t6542 + t1877 * t26740 * t25 / 2.0_f64 - t1877 * t26744 * t6671 / 2.0_f64 + t1877 * t7845 * t606 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t25366 - t1877 * t24339 * t7545 / 2.0_f64 + t26756 * t25375 - t1877 * t7114 * t25377 / 2.0_f64 - t1877 * t7114 * t25381 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t25385 + t1877 * t7110 * t1408 / 2.0_f64 - t1877 * t7114 * t25392 / 2.0_f64 + t26774;
    (t26774, t26775)
}
