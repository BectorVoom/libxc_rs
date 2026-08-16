//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 433/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk433(t2300: f64, t772: f64, t132: f64, t268: f64, t770: f64, t798: f64, t2216: f64, t793: f64, t297: f64, t966: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2301 = t772 * t2300;
    let t2304 = t132 * t268;
    let t2305 = t2304 * t770;
    let t2308 = t2304 * t798;
    let t2311 = t2216 * t793;
    let t2314 = t297 * t966;
    let t2315 = t875 * t875;
    (t2301, t2305, t2308, t2311, t2314, t2315)
}
