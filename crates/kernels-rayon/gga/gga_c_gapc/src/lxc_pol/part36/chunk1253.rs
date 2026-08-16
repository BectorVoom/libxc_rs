//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1253/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1253(t3643: f64, t760: f64, t10350: f64, t11678: f64, t11679: f64, t24202: f64, t11656: f64, t11658: f64, t24181: f64, t11214: f64, t11663: f64, t6853: f64) -> (f64, f64, f64, f64, f64) {
    let t35734 = t3643 * t760;
    let t35736 = t35734 * t11678 * t10350;
    let t35738 = t11679 * t24202;
    let t35741 = t24181 * t11656 * t11658;
    let t35745 = t11214 * t760 * t6853 * t11663;
    (t35734, t35736, t35738, t35741, t35745)
}
