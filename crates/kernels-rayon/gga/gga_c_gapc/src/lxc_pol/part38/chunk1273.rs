//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1273/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1273(t190: f64, t23608: f64, t24110: f64, t35729: f64, t3643: f64, t760: f64, t10350: f64, t11678: f64, t11679: f64, t24202: f64, t11656: f64, t11658: f64, t24181: f64) -> (f64, f64, f64, f64, f64) {
    let t35732 = t35729 * t23608 * t190 * t24110;
    let t35734 = t3643 * t760;
    let t35736 = t35734 * t11678 * t10350;
    let t35738 = t11679 * t24202;
    let t35741 = t24181 * t11656 * t11658;
    (t35732, t35734, t35736, t35738, t35741)
}
