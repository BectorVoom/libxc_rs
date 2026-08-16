//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2511/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2511(t43780: f64, t43782: f64, t43784: f64, t43786: f64, t43788: f64, t43816: f64, t43895: f64, t50976: f64, t50978: f64, t50987: f64, t50990: f64, t50994: f64) -> f64 {
    let t50996 = t43895 - 0.8585111111111111111e-1_f64 * t50976 - 0.73586666666666666668e-1_f64 * t50978 + 0.40256666666666666667e0_f64 * t43780 + 0.80513333333333333335e0_f64 * t43782 + 0.40256666666666666668e0_f64 * t43784 - 0.60385000000000000002e0_f64 * t43786 - 0.10064166666666666667e0_f64 * t43788 - 0.93932222222222222223e0_f64 * t43816 + 0.11038e0_f64 * t50987 + 0.44152e0_f64 * t50990 - 0.36231e1_f64 * t50994;
    t50996
}
