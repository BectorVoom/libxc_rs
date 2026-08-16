//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1050/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1050(t31217: f64, t6368: f64, t21321: f64, t8244: f64, t31146: f64, t4231: f64, t4230: f64, t30494: f64, t6322: f64, t6321: f64, t4204: f64, t4203: f64) -> (f64, f64, f64, f64, f64) {
    let t31218 = t6368 * t31217;
    let t31220 = t21321 * t8244;
    let t31222 = t4231 * t31146;
    let t31223 = t4230 * t31222;
    let t31225 = t6322 * t30494;
    let t31226 = t6321 * t31225;
    let t31228 = t4204 * t31146;
    let t31229 = t4203 * t31228;
    (t31218, t31220, t31223, t31226, t31229)
}
