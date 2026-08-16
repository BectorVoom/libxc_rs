//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1466/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1466(t1606: f64, t698: f64, t973: f64, t1043: f64, t2770: f64, t10277: f64, t3061: f64, t10216: f64, t10969: f64, t135: f64, t4608: f64, t10868: f64, t1539: f64, t248: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14159 = t698 * t1606;
    let t14160 = t973 * t14159;
    let t14164 = t1043 * t2770;
    let t14172 = t3061 * t10277;
    let t14187 = t10969 * t10216;
    let t14192 = t135 * t4608;
    let t14194 = t973 * t14192 / 432.0_f64;
    let t14202 = t248 * t10868 * t1539;
    (t14159, t14160, t14164, t14172, t14187, t14194, t14202)
}
