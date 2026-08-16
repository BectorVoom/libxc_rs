//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1277/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1277(t11492: f64, t34468: f64, t11317: f64, t2973: f64, t3140: f64, t34040: f64, t27935: f64, t27940: f64, t11598: f64, t8765: f64, t11387: f64, t19916: f64, t5553: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35135 = t34468 * t11492;
    let t35137 = t2973 * t11317;
    let t35139 = t34040 * t3140;
    let t35141 = t27935 * t35139 * t27940;
    let t35143 = t11598 * t8765;
    let t35146 = t5553 * t11387 * t19916;
    (t35135, t35137, t35139, t35141, t35143, t35146)
}
