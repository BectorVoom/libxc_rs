//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2241/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2241(t1041: f64, t10868: f64, t248: f64, t5681: f64, t13965: f64, t4641: f64, t17659: f64, t3048: f64, t14207: f64, t4630: f64, t13969: f64, t17717: f64, t3039: f64) -> (f64, f64, f64, f64, f64) {
    let t62137 = t1041 * t248 * t10868 * t5681;
    let t62148 = t4641 * t13965;
    let t62150 = t3048 * t17659;
    let t62152 = t14207 * t4630;
    let t62164 = t3039 * t13969 * t17717;
    (t62137, t62148, t62150, t62152, t62164)
}
