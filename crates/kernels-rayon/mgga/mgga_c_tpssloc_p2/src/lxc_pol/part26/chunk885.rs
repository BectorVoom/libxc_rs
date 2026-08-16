//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 885/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk885(t10445: f64, t354: f64, t1036: f64, t3089: f64, t248: f64, t2780: f64, t3051: f64, t1041: f64, t121: f64, t3061: f64, t2771: f64, t10321: f64, t1044: f64) -> (f64, f64, f64, f64, f64) {
    let t10446 = t354 * t10445;
    let t10449 = t3089 * t1036;
    let t10454 = t248 * t3051 * t2780;
    let t10455 = t1041 * t10454;
    let t10457 = t121 * t3061;
    let t10459 = t248 * t10457 * t2771;
    let t10460 = t1041 * t10459;
    let t10463 = t248 * t1044 * t10321;
    (t10446, t10449, t10455, t10460, t10463)
}
