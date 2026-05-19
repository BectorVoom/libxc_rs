//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 847/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk847<F: Float>(t16333: F, t16334: F, t16343: F, t16349: F, t124: F, t16221: F, t3412: F, t4595: F, t16287: F, t121: F, t1268: F, t13124: F, t3406: F, t3411: F, t4643: F, t4646: F, t641: F, t6855: F, t9735: F) -> (F, F, F, F, F) {
    let t16351 = t16333 + t16334 + t16343 + t16349;
    let t16361 = t124 * t16221;
    let t16364 = t3412 * t4595;
    let t16367 = t124 * t16287;
    let t16370 = -F::cast_from(0.12897460341341234505e3_f64) * t16351 * t121 * t124 + F::cast_from(0.11607714307207111054e4_f64) * t13124 * t1268 - F::cast_from(0.46430857228828444218e4_f64) * t9735 * t4643 + F::cast_from(0.11607714307207111054e4_f64) * t3406 * t4646 + F::cast_from(0.7738476204804740703e4_f64) * t6855 * t16361 - F::cast_from(0.46430857228828444218e4_f64) * t3411 * t16364 + F::cast_from(0.38692381024023703515e3_f64) * t641 * t16367;
    (t16351, t16361, t16364, t16367, t16370)
}
