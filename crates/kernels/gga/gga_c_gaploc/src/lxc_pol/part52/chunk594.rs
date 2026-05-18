//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 594/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk594<F: Float>(t3549: F, t501: F, t3553: F, t605: F, t1016: F, t2902: F, t3599: F, t2754: F, t2854: F, t1445: F, t11241: F, t11168: F) -> (F, F, F, F, F, F, F) {
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
