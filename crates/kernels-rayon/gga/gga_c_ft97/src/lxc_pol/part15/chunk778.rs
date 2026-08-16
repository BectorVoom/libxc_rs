//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 778/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk778(t21399: f64, t676: f64, t27: f64, t89: f64, t1091: f64, t4934: f64, t9770: f64, t446: f64, t1131: f64, t4969: f64, t2354: f64, t20489: f64, t669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21400 = t676 * t21399;
    let t21402 = t89 * t27 * t21400;
    let t21404 = t1091 * t4934;
    let t21405 = t9770 * t21404;
    let t21406 = t446 * t21405;
    let t21408 = t4969 * t1131;
    let t21409 = t2354 * t21408;
    let t21410 = t446 * t21409;
    let t21412 = t669 * t20489;
    (t21400, t21402, t21404, t21405, t21406, t21408, t21409, t21410, t21412)
}
