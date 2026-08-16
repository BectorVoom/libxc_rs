//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 628/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk628(t3549: f64, t501: f64, t3553: f64, t605: f64, t1016: f64, t2902: f64, t3599: f64, t2754: f64, t2854: f64, t1445: f64, t11241: f64, t11168: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11288 = t3549 * t501;
    let t11298 = t3553 * t605;
    let t11301 = t1016 * t2902;
    let t11305 = t3599 * t605;
    let t11308 = t2854 * t2754;
    let t11309 = t1445 * t11308;
    let t11312 = t1445 * t11241;
    let t11315 = t1445 * t11168;
    (t11288, t11298, t11301, t11305, t11309, t11312, t11315)
}
