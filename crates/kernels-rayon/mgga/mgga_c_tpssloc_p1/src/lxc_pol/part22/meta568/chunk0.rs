//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2074/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2074(t2402: f64, t973: f64, t986: f64, t10213: f64, t135: f64, t41961: f64, t697: f64, t976: f64, t984: f64, t13797: f64, t10216: f64, t343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42903 = t973 * t2402 * t986;
    let t42972 = t135 * t10213;
    let t43002 = 220.0_f64 / 81.0_f64 * t41961;
    let t43052 = t697 * t976;
    let t43053 = t43052 * t984;
    let t43069 = t13797 * t984;
    let t43070 = t343 * t10216;
    (t42903, t42972, t43002, t43052, t43053, t43069, t43070)
}
