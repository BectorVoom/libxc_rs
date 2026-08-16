//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 999/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk999(t4917: f64, t875: f64, t10503: f64, t2881: f64, t18514: f64, t4140: f64, t18497: f64, t4265: f64, t11593: f64, t15180: f64, t15190: f64, t15206: f64, t1901: f64, t19432: f64, t19437: f64, t19442: f64, t19446: f64, t19449: f64, t19451: f64, t19453: f64, t19457: f64, t19462: f64, t446: f64) -> (f64, f64) {
    let t19465 = t4917 * t875;
    let t19466 = t10503 * t19465;
    let t19467 = t2881 * t19466;
    let t19470 = t4140 * t18514;
    let t19471 = t2881 * t19470;
    let t19474 = t4265 * t18497;
    let t19475 = t2881 * t19474;
    let t19478 = -2.0_f64 * t446 * t19432 + 4.0_f64 / 3.0_f64 * t446 * t19437 - 4.0_f64 / 27.0_f64 * t15180 - t446 * t19442 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t19446 - t15190 + 2.0_f64 / 9.0_f64 * t19449 + t19451 / 9.0_f64 + t19453 / 9.0_f64 + t15206 - 4.0_f64 / 9.0_f64 * t11593 * t19457 - 2.0_f64 / 9.0_f64 * t1901 * t19462 - 2.0_f64 / 9.0_f64 * t1901 * t19467 - 2.0_f64 / 3.0_f64 * t1901 * t19471 - 8.0_f64 / 9.0_f64 * t11593 * t19475;
    (t19465, t19478)
}
