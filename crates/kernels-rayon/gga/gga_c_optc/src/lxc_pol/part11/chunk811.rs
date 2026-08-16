//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 811/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk811(t1218: f64, t5274: f64, t1217: f64, t5238: f64, t5241: f64, t8: f64, t5236: f64, t1113: f64, t190: f64, t136: f64, t3: f64, t496: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15015 = t1218 * t5274;
    let t15016 = t1217 * t15015;
    let t15063 = t5238 * t5241 * t8;
    let t15064 = t5236 * t15063;
    let t15065 = t1113 * t190;
    let t15066 = t15065 * t136;
    let t15067 = t3 * t496;
    (t15015, t15016, t15063, t15064, t15066, t15067)
}
