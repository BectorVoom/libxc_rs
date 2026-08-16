//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 786/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk786(t12960: f64, t31051: f64, t10473: f64, t2478: f64, t6576: f64, t34688: f64, t9272: f64, t9273: f64, t18313: f64, t31119: f64, t3394: f64, t35180: f64, t9562: f64) -> (f64, f64, f64, f64, f64) {
    let t41645 = t31051 * t12960;
    let t41649 = t6576 * t10473 * t2478;
    let t41656 = t9272 * t34688 * t9273;
    let t41660 = t31119 * t18313 * t3394 * t9273;
    let t41666 = t35180 * t9562;
    (t41645, t41649, t41656, t41660, t41666)
}
