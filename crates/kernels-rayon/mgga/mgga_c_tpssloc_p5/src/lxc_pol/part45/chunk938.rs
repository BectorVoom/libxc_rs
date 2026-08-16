//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 938/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk938(t23110: f64, t23185: f64, t30685: f64, t1880: f64, t1894: f64, t214: f64, t23150: f64, t23012: f64, t8357: f64, t30690: f64, t6547: f64, t23030: f64, t30681: f64) -> (f64, f64, f64, f64, f64) {
    let t112983 = t23185 * t23110 * t30685;
    let t112984 = 0.16449340668482264365e-1_f64 * t112983;
    let t112988 = 0.16449340668482264365e-1_f64 * t1880 * t214 * t1894 * t23150;
    let t112990 = 0.12793931631041761173e0_f64 * t23012 * t8357;
    let t112991 = t6547 * t30690;
    let t112992 = 0.76763589786250567036e-1_f64 * t112991;
    let t112995 = 0.52089578783527170489e-1_f64 * t23030 * t30681;
    (t112984, t112988, t112990, t112992, t112995)
}
