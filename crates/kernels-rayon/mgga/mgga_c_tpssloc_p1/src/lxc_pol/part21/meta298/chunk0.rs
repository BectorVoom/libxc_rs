//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1623/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1623(t2775: f64, t283: f64, t61: f64, t135: f64, t3142: f64, t973: f64, t3147: f64, t3152: f64, t248: f64, t3101: f64, t3132: f64, t3130: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10969 = 1.0_f64 / t283 / t2775;
    let t10970 = t61 * t10969;
    let t10981 = t135 * t3142;
    let t10982 = t973 * t10981;
    let t10984 = t135 * t3147;
    let t10985 = t973 * t10984;
    let t10993 = t135 * t3152;
    let t10994 = t973 * t10993;
    let t11002 = t248 * t3101 * t3132;
    let t11003 = t3130 * t11002;
    (t10969, t10970, t10982, t10985, t10994, t11002, t11003)
}
