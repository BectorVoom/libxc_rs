//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1182/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1182(t11495: f64, t1723: f64, t11500: f64, t11356: f64, t3060: f64, t9262: f64, t11303: f64, t19530: f64, t11302: f64, t5285: f64, t5218: f64, t33273: f64, t5260: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34597 = t11495 * t1723;
    let t34599 = t11500 * t1723;
    let t34602 = t3060 * t11356 * t9262;
    let t34605 = t11303 * t19530;
    let t34607 = t5285 * t11302;
    let t34608 = t34607 * t5218;
    let t34611 = t5260 * t33273 * t676;
    (t34597, t34599, t34602, t34605, t34607, t34608, t34611)
}
