//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 969/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk969<F: Float>(t225: F, t7192: F, t7179: F, t22692: F, t22717: F, t22725: F, t1338: F, t7191: F, t22923: F, t22925: F, t532: F, t7216: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24082 = t7192 * t225;
    let t24095 = t7179 * t225;
    let t24099 = F::cast_from(0.16449340668482264365e-1_f64) * t22692;
    let t24108 = F::cast_from(0.12793931631041761173e0_f64) * t22717;
    let t24110 = F::cast_from(0.52089578783527170489e-1_f64) * t22725;
    let t24116 = t1338 * t7191;
    let t24156 = F::cast_from(0.12793931631041761173e0_f64) * t22923;
    let t24157 = F::cast_from(0.52089578783527170489e-1_f64) * t22925;
    let t24175 = t532 * t7216;
    (t24082, t24095, t24099, t24108, t24110, t24116, t24156, t24157, t24175)
}
