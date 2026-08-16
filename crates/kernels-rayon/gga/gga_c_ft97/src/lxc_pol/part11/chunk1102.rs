//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1102/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1102(t10912: f64, t2253: f64, t3628: f64, t895: f64, t10904: f64, t230: f64, t2440: f64, t900: f64, t10864: f64, t10894: f64, t10905: f64, t14487: f64, t18862: f64, t2265: f64, t2409: f64, t2923: f64, t2938: f64, t2939: f64, t2951: f64, t41454: f64, t41464: f64, t41473: f64, t41482: f64, t41490: f64, t4334: f64, t4342: f64, t505: f64, t631: f64, t684: f64, t898: f64, t904: f64, t9572: f64, t9587: f64, t9596: f64) -> f64 {
    let t43094 = t2253 * t10912;
    let t43101 = t3628 * t895;
    let t43109 = t230 * t10904;
    let t43122 = t2440 * t900;
    let t43140 = 12.0_f64 * t2265 * t18862 * t505 * t2951 * t904 + 12.0_f64 * t43094 - 6.0_f64 * t631 * t898 * t2938 * t10894 * t904 - 160.0_f64 / 81.0_f64 * t43101 + 6.0_f64 * t2265 * t4342 * t41482 - 4.0_f64 / 3.0_f64 * t2265 * t4342 * t41490 - 16.0_f64 * t2265 * t43109 * t684 * t10905 - 12.0_f64 * t2265 * t10864 * t2409 * t2939 - 8.0_f64 * t2265 * t2923 * t9587 * t904 - 16.0_f64 / 27.0_f64 * t2265 * t43122 * t9572 * t904 - 2.0_f64 * t2265 * t4334 * t41464 + 2.0_f64 / 9.0_f64 * t2265 * t4334 * t41473 + 4.0_f64 / 9.0_f64 * t2265 * t14487 * t41454 - 4.0_f64 / 3.0_f64 * t2265 * t2923 * t9596 * t904;
    t43140
}
