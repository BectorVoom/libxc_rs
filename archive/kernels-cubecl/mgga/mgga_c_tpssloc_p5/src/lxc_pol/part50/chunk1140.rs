//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1140/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1140<F: Float>(t23110: F, t23185: F, t30685: F, t23012: F, t8357: F, t30690: F, t6547: F, t23030: F, t30681: F, t30689: F, t6562: F, t794: F) -> (F, F, F, F, F) {
    let t112983 = t23185 * t23110 * t30685;
    let t112990 = F::cast_from(0.12793931631041761173e0_f64) * t23012 * t8357;
    let t112991 = t6547 * t30690;
    let t112995 = F::cast_from(0.52089578783527170489e-1_f64) * t23030 * t30681;
    let t112997 = t6562 * t794 * t30689;
    (t112983, t112990, t112991, t112995, t112997)
}
