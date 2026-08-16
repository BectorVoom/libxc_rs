//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1025/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1025(t18568: f64, t4624: f64, t4697: f64, t4640: f64, t4689: f64, t1365: f64, t1447: f64, t472: f64, t1218: f64, t542: f64, t1422: f64, t1448: f64, t4: f64) -> (f64, f64, f64, f64, f64) {
    let t18571 = 0.1926377843805564792e1_f64 * t18568 * t4697 * t4624;
    let t18574 = 0.13012297059337829057e0_f64 * t18568 * t4689 * t4640;
    let t18577 = 0.67471169937307261776e-1_f64 * t1447 * t1365 * t472;
    let t18580 = 0.86748647062252193713e-1_f64 * t1447 * t542 * t1218;
    let t18582 = t1422 * t4 * t1448;
    (t18571, t18574, t18577, t18580, t18582)
}
