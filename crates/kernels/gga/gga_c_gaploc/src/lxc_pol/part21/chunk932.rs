//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 932/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk932<F: Float>(t12214: F, t1457: F, t2089: F, t3720: F, t723: F, t1445: F, t325: F) -> (F, F, F, F, F) {
    let t12215 = t1457 * t12214;
    let t12218 = t2089 * t3720;
    let t12219 = t12218 * t723;
    let t12220 = t1445 * t12219;
    let t12223 = t325 * t3720;
    (t12215, t12218, t12219, t12220, t12223)
}
