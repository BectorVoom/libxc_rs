//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1414/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1414(t209: f64, t36420: f64, t36449: f64, t38539: f64, t38542: f64, t38545: f64, t38548: f64, t38552: f64, t34361: f64, t34373: f64, t36906: f64, t36907: f64, t36908: f64, t36909: f64, t36910: f64, t36911: f64, t36913: f64, t36914: f64, t36915: f64) -> (f64, f64) {
    let t38556 = (t38539 + t38542 + t38545 + t38548 + t36420 + t38552 + t36449) * t209;
    let t38565 = t36906 + t36907 - t36908 - t36909 + t36910 + t36911 - 0.56912804804009946682e-7_f64 * t34361 + t36913 + t36914 - t36915 + 0.68360384691762319208e-5_f64 * t34373;
    (t38556, t38565)
}
