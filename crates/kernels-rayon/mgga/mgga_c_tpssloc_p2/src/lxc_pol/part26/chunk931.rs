//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 931/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk931(t10987: f64, t974: f64, t135: f64, t3152: f64, t973: f64, t2770: f64, t976: f64, t9288: f64, t248: f64, t3101: f64, t3132: f64, t3130: f64) -> (f64, f64, f64, f64) {
    let t10988 = t974 * t10987;
    let t10993 = t135 * t3152;
    let t10994 = t973 * t10993;
    let t10996 = t976 * t2770;
    let t10997 = t10996 * t9288;
    let t10998 = t974 * t10997;
    let t11002 = t248 * t3101 * t3132;
    let t11003 = t3130 * t11002;
    (t10988, t10994, t10998, t11003)
}
