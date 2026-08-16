//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 863/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk863(t3069: f64, t4669: f64, t1612: f64, t3082: f64, t1606: f64, t698: f64, t973: f64, t1043: f64, t2770: f64, t10277: f64, t3061: f64, t10216: f64, t10969: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13995 = t4669 * t3069;
    let t14117 = t1612 * t3082;
    let t14159 = t698 * t1606;
    let t14160 = t973 * t14159;
    let t14164 = t1043 * t2770;
    let t14172 = t3061 * t10277;
    let t14187 = t10969 * t10216;
    (t13995, t14117, t14160, t14164, t14172, t14187)
}
