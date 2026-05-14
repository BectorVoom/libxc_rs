//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 562/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk562<F: Float>(t2754: F, t2787: F, t2343: F, t1437: F, t3565: F, t2765: F, t2792: F, t3531: F, t535: F, t3529: F, t599: F) -> (F, F, F, F, F, F) {
    let t11241 = t2787 * t2754;
    let t11242 = t2343 * t11241;
    let t11245 = t3565 * t1437;
    let t11248 = t2765 * t2792;
    let t11251 = t535 * t3531;
    let t11254 = t599 * t3529;
    (t11241, t11242, t11245, t11248, t11251, t11254)
}
