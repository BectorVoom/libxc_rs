//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 748/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk748(t2417: f64, t810: f64, t2409: f64, t3067: f64, t829: f64, t830: f64, t2209: f64, t337: f64, t2182: f64, t831: f64, t2118: f64, t2365: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6138 = t810 * t2417;
    let t6140 = t2409 * t3067 * t6138;
    let t6143 = t3067 * t2417;
    let t6145 = t829 * t830 * t6143;
    let t6148 = t2209 * t337;
    let t6149 = t831 * t2182;
    let t6151 = t6148 * t830 * t6149;
    let t6154 = t2118 * t2365;
    (t6138, t6140, t6145, t6148, t6151, t6154)
}
