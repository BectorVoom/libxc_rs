//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1366/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1366(t11835: f64, t7310: f64, t11647: f64, t2141: f64, t1184: f64, t607: f64, t24682: f64, t460: f64, t3469: f64, t52: f64, t3475: f64, t10469: f64, t11715: f64, t478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t86184 = t7310 * t11835;
    let t86191 = t2141 * t11647 / 5184.0_f64;
    let t86192 = t607 * t1184;
    let t86194 = t24682 * t86192 * t460;
    let t86197 = t52 * t3469;
    let t86199 = t24682 * t86197 * t460;
    let t86202 = t52 * t3475;
    let t86204 = t24682 * t86202 * t460;
    let t86208 = t10469 * t11715 * t478;
    (t86184, t86191, t86192, t86194, t86197, t86199, t86202, t86204, t86208)
}
