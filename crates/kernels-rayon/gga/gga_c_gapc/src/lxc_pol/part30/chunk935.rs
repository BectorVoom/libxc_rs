//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 935/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk935(t1912: f64, t3717: f64, t5285: f64, t11329: f64, t3144: f64, t3709: f64, t8885: f64, t8448: f64, t9059: f64, t8784: f64, t520: f64, t8788: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11373 = t5285 * t3717 * t1912;
    let t11375 = t11329 * t3144;
    let t11377 = t3709 * t8885;
    let t11379 = t9059 * t8448;
    let t11380 = t8784 * t11379;
    let t11381 = t520 * t8788;
    (t11373, t11375, t11377, t11379, t11380, t11381)
}
