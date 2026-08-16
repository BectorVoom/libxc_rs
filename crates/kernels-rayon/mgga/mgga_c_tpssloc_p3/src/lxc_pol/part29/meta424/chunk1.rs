//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1712/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1712(t1388: f64, t1799: f64, t3792: f64, t5286: f64, t576: f64, t671: f64, t1874: f64, t9348: f64, t4034: f64, t6535: f64, t107: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19577 = t1799 * t1388;
    let t19735 = t3792 * t5286;
    let t20173 = t576 * t671;
    let t22460 = 2.0_f64 * t9348 * t1874;
    let t22467 = 4.0_f64 * t4034 * t6535;
    let t22468 = t240 * t107;
    (t19577, t19735, t20173, t22460, t22467, t22468)
}
