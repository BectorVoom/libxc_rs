//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1060/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1060(t12689: f64, t459: f64, t1294: f64, t3790: f64, t3737: f64, t1284: f64, t3552: f64, t1204: f64, t3766: f64, t3153: f64, t3588: f64, t5480: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12690 = t12689 * t459;
    let t12695 = t1294 * t3790;
    let t12696 = t3737 * t12695;
    let t12699 = t3552 * t1284;
    let t12702 = t1204 * t3766;
    let t12705 = t3588 * t3153;
    let t12706 = t12705 * t5480;
    (t12690, t12696, t12699, t12702, t12705, t12706)
}
