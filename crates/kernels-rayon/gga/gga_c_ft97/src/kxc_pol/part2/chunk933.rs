//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 933/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk933(t13296: f64, t4342: f64, t13301: f64, t4339: f64, t8675: f64, t4343: f64, t4335: f64, t12143: f64, t14421: f64, t14423: f64, t14426: f64, t14429: f64, t14431: f64, t14434: f64, t14439: f64, t14442: f64, t14445: f64, t14448: f64, t14451: f64, t14455: f64, t14460: f64, t14464: f64, t14468: f64, t2265: f64, t3628: f64, t631: f64) -> f64 {
    let t14471 = t4342 * t13296;
    let t14474 = t4342 * t13301;
    let t14478 = 4.0_f64 / 9.0_f64 * t8675 * t4339;
    let t14480 = 4.0_f64 / 9.0_f64 * t8675 * t4343;
    let t14482 = 2.0_f64 / 27.0_f64 * t8675 * t4335;
    let t14483 = t14421 + t14423 + 6.0_f64 * t631 * t14426 + 13.0_f64 / 9.0_f64 * t14429 + 5.0_f64 / 27.0_f64 * t14431 - 3.0_f64 / 2.0_f64 * t631 * t14434 - 3.0_f64 * t631 * t14439 - t3628 * t14442 / 3.0_f64 + 5.0_f64 / 9.0_f64 * t14445 - t14448 - 2.0_f64 / 3.0_f64 * t2265 * t14451 - t2265 * t14455 / 3.0_f64 - t2265 * t14460 / 9.0_f64 - t2265 * t14464 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t12143 * t14468 - t2265 * t14471 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t12143 * t14474 + t14478 + t14480 - t14482;
    t14483
}
