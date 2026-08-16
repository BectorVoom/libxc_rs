//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 959/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk959(t1820: f64, t2559: f64, t4352: f64, t4957: f64, t562: f64, t1775: f64, t1806: f64, t617: f64, t661: f64, t1620: f64, t1724: f64, t7216: f64) -> (f64, f64, f64) {
    let t17783 = 64.0_f64 / 9.0_f64 * t1820 * t2559 * t562 * t4957 * t4352;
    let t17785 = 8.0_f64 / 5.0_f64 * t1775 * t1806;
    let t17786 = t661 * t617;
    let t17790 = 32.0_f64 / 5.0_f64 * t1620 * t7216 * t17786 * t1724;
    (t17783, t17785, t17790)
}
