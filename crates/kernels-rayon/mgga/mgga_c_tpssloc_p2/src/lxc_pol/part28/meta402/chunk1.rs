//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1561/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1561(t1388: f64, t1799: f64, t3792: f64, t5286: f64, t576: f64, t671: f64, t107: f64, t240: f64, t625: f64, t656: f64, t666: f64, t2331: f64, t63: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19577 = t1799 * t1388;
    let t19735 = t3792 * t5286;
    let t20173 = t576 * t671;
    let t22468 = t240 * t107;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22472 = 2.0_f64 / 3.0_f64 * t22471;
    let t22473 = t63 * t2331;
    (t19577, t19735, t20173, t22468, t22470, t22471, t22472, t22473)
}
