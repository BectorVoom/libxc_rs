//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 847/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk847(t16333: f64, t16334: f64, t16343: f64, t16349: f64, t124: f64, t16221: f64, t3412: f64, t4595: f64, t16287: f64, t121: f64, t1268: f64, t13124: f64, t3406: f64, t3411: f64, t4643: f64, t4646: f64, t641: f64, t6855: f64, t9735: f64) -> (f64, f64, f64, f64, f64) {
    let t16351 = t16333 + t16334 + t16343 + t16349;
    let t16361 = t124 * t16221;
    let t16364 = t3412 * t4595;
    let t16367 = t124 * t16287;
    let t16370 = -0.12897460341341234505e3_f64 * t16351 * t121 * t124 + 0.11607714307207111054e4_f64 * t13124 * t1268 - 0.46430857228828444218e4_f64 * t9735 * t4643 + 0.11607714307207111054e4_f64 * t3406 * t4646 + 0.7738476204804740703e4_f64 * t6855 * t16361 - 0.46430857228828444218e4_f64 * t3411 * t16364 + 0.38692381024023703515e3_f64 * t641 * t16367;
    (t16351, t16361, t16364, t16367, t16370)
}
