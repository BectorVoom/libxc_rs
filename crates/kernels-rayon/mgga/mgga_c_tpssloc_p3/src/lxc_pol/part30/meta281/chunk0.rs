//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1272/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1272(t1998: f64, t7708: f64, t6926: f64, t1339: f64, t1825: f64, t6936: f64, t1814: f64, t2002: f64, t559: f64, t1827: f64, t6945: f64, t1831: f64, t6952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7709 = t1998 * t7708;
    let t7710 = t6926 * t7709;
    let t7712 = t1339 * t1825;
    let t7713 = t6936 * t7712;
    let t7715 = t1814 * t2002;
    let t7716 = t7715 * t559;
    let t7718 = t6945 * t1827;
    let t7720 = t6952 * t1831;
    (t7709, t7710, t7712, t7713, t7715, t7716, t7718, t7720)
}
