//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 824/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk824<F: Float>(t2035: F, t5009: F, t19038: F, t287: F, t5014: F, t2724: F, t5260: F, t1200: F, t7606: F, t19106: F, t800: F, t4092: F, t70462: F, t19233: F, t1771: F, t5360: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t70474 = t2035 * t5009;
    let t70475 = t19038 * t70474;
    let t70476 = t5014 * t287;
    let t70487 = t2724 * t5260;
    let t70497 = t1200 * t7606;
    let t70550 = t800 * t19106;
    let t70653 = t4092 * t70462;
    let t70671 = t19233 * t287;
    let t70779 = t4092 * t19106;
    let t70799 = t1771 * t5360;
    (t70474, t70475, t70476, t70487, t70497, t70550, t70653, t70671, t70779, t70799)
}
