//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1004/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1004(t1849: f64, t601: f64, t6347: f64, t6405: f64, t2002: f64, t518: f64, t596: f64, t84: f64) -> (f64, f64, f64) {
    let t22111 = 0.62336721237753107879e3_f64 * t601 * t6405 * t1849 * t6347;
    let t22115 = 0.18989760778855128827e-2_f64 * t596 * t518 * t2002 * t84;
    let t22120 = t1849 * t1849;
    (t22111, t22115, t22120)
}
