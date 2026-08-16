//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1346/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1346(t42110: f64, t42113: f64, t76637: f64, t959: f64, t17934: f64, t5804: f64, t5694: f64, t42100: f64, t42102: f64, t5695: f64, t60357: f64, t21268: f64, t49489: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76641 = 0.91082604192152556044e5_f64 * t959 * t42110 * t76637 * t42113;
    let t76643 = 0.70178683471615754484e1_f64 * t17934 * t5804;
    let t76644 = t5694 * t5694;
    let t76647 = 0.24955700379505800916e5_f64 * t42100 * t76644 * t42102;
    let t76652 = 12.0_f64 * t60357 * t5695;
    let t76654 = 0.3859675079686208416e3_f64 * t49489 * t21268;
    (t76641, t76643, t76644, t76647, t76652, t76654)
}
