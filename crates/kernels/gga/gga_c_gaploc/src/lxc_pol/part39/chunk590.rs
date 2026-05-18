//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 590/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk590<F: Float>(t1445: F, t9954: F, t1457: F, t9604: F, t9591: F, t1: F, t3234: F, t106: F, t316: F, t3276: F, t773: F, t1645: F, t2586: F) -> (F, F, F, F, F, F) {
    let t9955 = t1445 * t9954;
    let t9958 = t1457 * t9604;
    let t9961 = t1457 * t9591;
    let t9964 = t3234 * t1;
    let t9965 = t9964 * t106;
    let t9966 = t9965 * t316;
    let t9969 = t773 * t3276;
    let t9972 = t1645 * t2586;
    (t9955, t9958, t9961, t9966, t9969, t9972)
}
