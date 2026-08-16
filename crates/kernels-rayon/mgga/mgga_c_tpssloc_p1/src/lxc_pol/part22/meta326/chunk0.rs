//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1512/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1512(t1788: f64, t2221: f64, t2223: f64, t11987: f64, t1408: f64, t2: f64, t3704: f64, t12000: f64, t1649: f64, t3711: f64, t225: f64, t5213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15984 = t2221 * t1788;
    let t15986 = t2223 * t1788;
    let t15989 = t11987 * t1408;
    let t15992 = t3704 * t2;
    let t16003 = t12000 * t1649;
    let t16006 = t3711 * t2;
    let t16022 = t5213 * t225;
    (t15984, t15986, t15989, t15992, t16003, t16006, t16022)
}
