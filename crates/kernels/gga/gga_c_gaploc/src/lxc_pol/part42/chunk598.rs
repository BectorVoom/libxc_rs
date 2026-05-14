//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 598/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk598<F: Float>(t1: F, t3720: F, t106: F, t316: F, t3732: F, t773: F, t835: F, t723: F, t1457: F, t2089: F, t1445: F, t325: F) -> (F, F, F, F, F, F, F) {
    let t12205 = t3720 * t1;
    let t12206 = t12205 * t106;
    let t12207 = t12206 * t316;
    let t12210 = t773 * t3732;
    let t12213 = t835 * t3720;
    let t12214 = t12213 * t723;
    let t12215 = t1457 * t12214;
    let t12218 = t2089 * t3720;
    let t12219 = t12218 * t723;
    let t12220 = t1445 * t12219;
    let t12223 = t325 * t3720;
    (t12207, t12210, t12213, t12214, t12215, t12220, t12223)
}
