//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 960/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk960(t32100: f64, t921: f64, t2358: f64, t33959: f64, t27214: f64, t9253: f64, t10624: f64, t1382: f64, t1365: f64, t31558: f64, t6525: f64, t12963: f64, t1358: f64, t2299: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42513 = t32100 * t921;
    let t42517 = t33959 * t2358;
    let t42520 = 6.0_f64 * t27214 * t9253;
    let t42522 = t1382 * t10624 * t921;
    let t42529 = t6525 * t1365 * t31558;
    let t42533 = t1358 * t2299 * t12963 * t488;
    (t42513, t42517, t42520, t42522, t42529, t42533)
}
