//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 760/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk760(t10241: f64, t1359: f64, t544: f64, t31747: f64, t493: f64, t2925: f64, t299: f64, t11679: f64, t161: f64, t3601: f64, t830: f64, t723: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t35220 = t493 * t31747;
    let t35385 = t299 * t2925;
    let t35435 = t11679 * t161;
    let t35439 = t830 * t3601;
    let t35440 = t35439 * t161;
    let t35445 = t3601 * t723;
    (t35215, t35216, t35220, t35385, t35435, t35439, t35440, t35445)
}
