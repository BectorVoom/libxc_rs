//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 934/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk934<F: Float>(t4130: F, t41809: F, t4781: F, t590: F, t493: F, t1441: F, t1339: F, t41838: F, t1537: F, t18313: F, t18372: F, t41596: F) -> (F, F, F, F, F) {
    let t42047 = F::new(0.15337170381568299871e1) * t4781 * t4130 * t41809 * t590;
    let t42048 = t493 * t41809;
    let t42051 = F::new(0.1022478025437886658e1) * t1441 * t42048 * t590;
    let t42052 = t1339 * t41838;
    let t42054 = t1537 * t42052 * t590;
    let t42059 = F::new(0.25561950635947166451e1) * t1537 * t1339 * t41809 * t590;
    let t42064 = F::new(0.61348681526273199482e1) * t18372 * t18313 * t41596 * t590;
    (t42047, t42051, t42054, t42059, t42064)
}
