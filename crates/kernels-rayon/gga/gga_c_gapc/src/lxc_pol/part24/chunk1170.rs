//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1170/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1170(t11594: f64, t20604: f64, t11597: f64, t3001: f64, t9071: f64, t11598: f64, t9080: f64, t8848: f64, t19624: f64, t33148: f64, t5395: f64, t1030: f64, t33895: f64, t9249: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34550 = t11594 * t20604;
    let t34553 = t9071 * t11597 * t3001;
    let t34555 = t11598 * t9080;
    let t34557 = t11598 * t8848;
    let t34560 = t5395 * t33148 * t19624;
    let t34563 = t1030 * t33895 * t9249;
    (t34550, t34553, t34555, t34557, t34560, t34563)
}
