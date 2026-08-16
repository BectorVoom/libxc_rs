//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 924/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk924(t1902: f64, t794: f64, t23012: f64, t8357: f64, t23030: f64, t30681: f64, t22690: f64, t23171: f64, t30676: f64, t8332: f64, t8336: f64, t79: f64, t8306: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t112943 = t794 * t1902;
    let t112990 = 0.12793931631041761173e0_f64 * t23012 * t8357;
    let t112995 = 0.52089578783527170489e-1_f64 * t23030 * t30681;
    let t113005 = 0.16449340668482264365e-1_f64 * t23171 * t22690 * t30676;
    let t113038 = 0.12793931631041761173e0_f64 * t23012 * t8332;
    let t113045 = 0.12793931631041761173e0_f64 * t23012 * t8336;
    let t113875 = t8306 * t79;
    (t112943, t112990, t112995, t113005, t113038, t113045, t113875)
}
