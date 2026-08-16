//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 949/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk949(t1109: f64, t6095: f64, t1098: f64, t6072: f64, t6074: f64, t2454: f64, t4347: f64, t4339: f64, t8519: f64, t4598: f64, t992: f64, t1383: f64, t2519: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22688 = t1109 * t6095;
    let t22731 = t1098 * t6072 * t6074;
    let t22735 = t2454 * t4347;
    let t22743 = t8519 * t4339;
    let t22758 = t992 * t4598;
    let t22760 = t2519 * t1383;
    (t22688, t22731, t22735, t22743, t22758, t22760)
}
