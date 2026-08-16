//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2127/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2127(t23012: f64, t7485: f64, t1888: f64, t23270: f64, t2719: f64, t46488: f64, t25046: f64, t6579: f64, t1484: f64, t2717: f64, t22986: f64, t82099: f64) -> (f64, f64, f64, f64, f64) {
    let t86955 = t23012 * t7485;
    let t86961 = t1888 * t23270 * t46488 * t2719;
    let t86967 = t6579 * t25046;
    let t86968 = 0.76763589786250567036e-1_f64 * t86967;
    let t86969 = t2717 * t1484;
    let t86972 = t22986 * t23270 * t86969 * t2719;
    let t86983 = 0.52089578783527170489e-1_f64 * t82099;
    (t86955, t86961, t86968, t86972, t86983)
}
