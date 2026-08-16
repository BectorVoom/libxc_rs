//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 995/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk995(t18023: f64, t4290: f64, t4289: f64, t4282: f64, t3245: f64, t4300: f64, t5101: f64, t11900: f64, t5249: f64, t5256: f64, t1495: f64, t5239: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18183 = t4290 * t18023;
    let t18184 = t4289 * t18183;
    let t18187 = t4282 * t18023;
    let t18188 = t3245 * t18187;
    let t18190 = t4300 * t5101;
    let t18191 = t11900 * t18190;
    let t18194 = t5249 * t5256;
    let t18197 = t5239 * t1495;
    (t18183, t18184, t18187, t18188, t18190, t18191, t18194, t18197)
}
