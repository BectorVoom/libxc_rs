//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 911/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk911<F: Float>(t1902: F, t794: F, t23012: F, t8357: F, t23030: F, t30681: F, t22690: F, t23171: F, t30676: F, t8332: F, t8336: F, t79: F, t8306: F) -> (F, F, F, F, F, F, F) {
    let t112943 = t794 * t1902;
    let t112990 = F::cast_from(0.12793931631041761173e0_f64) * t23012 * t8357;
    let t112995 = F::cast_from(0.52089578783527170489e-1_f64) * t23030 * t30681;
    let t113005 = F::cast_from(0.16449340668482264365e-1_f64) * t23171 * t22690 * t30676;
    let t113038 = F::cast_from(0.12793931631041761173e0_f64) * t23012 * t8332;
    let t113045 = F::cast_from(0.12793931631041761173e0_f64) * t23012 * t8336;
    let t113875 = t8306 * t79;
    (t112943, t112990, t112995, t113005, t113038, t113045, t113875)
}
