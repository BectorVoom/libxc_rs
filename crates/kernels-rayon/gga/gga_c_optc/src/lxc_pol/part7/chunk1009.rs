//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1009/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1009(t1849: f64, t22100: f64, t601: f64, t6424: f64, t1847: f64, t588: f64, t6419: f64, t6347: f64, t6405: f64, t2002: f64, t518: f64, t596: f64, t84: f64) -> (f64, f64, f64, f64) {
    let t22103 = 0.61523382126046769581e4_f64 * t601 * t6424 * t1849 * t22100;
    let t22107 = 0.46785787179641632568e1_f64 * t601 * t1847 * t6419 * t588;
    let t22111 = 0.62336721237753107879e3_f64 * t601 * t6405 * t1849 * t6347;
    let t22115 = 0.18989760778855128827e-2_f64 * t596 * t518 * t2002 * t84;
    (t22103, t22107, t22111, t22115)
}
