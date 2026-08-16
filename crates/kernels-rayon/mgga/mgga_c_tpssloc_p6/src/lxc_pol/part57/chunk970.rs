//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 970/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk970(t114016: f64, t19871: f64, t3792: f64, t5248: f64, t120341: f64, t32721: f64, t19956: f64, t31170: f64, t550: f64, t6420: f64, t6936: f64, t6943: f64) -> (f64, f64, f64, f64, f64) {
    let t127283 = t114016 * t5248 * t19871 * t3792;
    let t127285 = t120341 * t32721;
    let t127289 = t31170 * t5248 * t19956 * t550;
    let t127293 = t31170 * t5248 * t19871 * t550;
    let t127296 = t6936 * t6943 * t6420;
    (t127283, t127285, t127289, t127293, t127296)
}
