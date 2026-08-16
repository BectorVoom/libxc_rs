//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 816/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk816(t43: f64, t50: f64, t13069: f64, t312: f64, t6906: f64, t1167: f64, t321: f64, t9772: f64, t12917: f64, t12919: f64, t12921: f64, t12923: f64, t12925: f64, t12927: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t13070 = t13069 * t312;
    let t13071 = 0.20538164420033333334e1_f64 * t6906;
    let t13073 = t321 * t9772 * t1167;
    let t13079 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t12917 - 2.0_f64 / 3.0_f64 * t12919 + 2.0_f64 / 3.0_f64 * t12921);
    let t13084 = piecewise3(t51, 0.0_f64, 8.0_f64 / 27.0_f64 * t12923 - 2.0_f64 / 3.0_f64 * t12925 + 2.0_f64 / 3.0_f64 * t12927);
    (t13070, t13071, t13073, t13079, t13084)
}
