//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1217/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1217(t10845: f64, t10864: f64, t1268: f64, t14514: f64, t14523: f64, t21355: f64, t21362: f64, t21369: f64, t21877: f64, t2265: f64, t2923: f64, t43164: f64, t4334: f64, t4342: f64, t4965: f64, t4969: f64, t4973: f64, t5457: f64, t5468: f64, t82112: f64, t88149: f64, t88153: f64, t88184: f64, t91330: f64, t992: f64) -> f64 {
    let t91387 = 12.0_f64 * t2265 * t14523 * t21877 - 4.0_f64 / 3.0_f64 * t2265 * t14514 * t91330 - 4.0_f64 / 3.0_f64 * t2265 * t2923 * t82112 * t992 - 2.0_f64 / 3.0_f64 * t2265 * t10845 * t4965 * t5468 - 2.0_f64 * t2265 * t4334 * t88184 - 2.0_f64 * t2265 * t2923 * t4973 * t5468 - 4.0_f64 / 3.0_f64 * t2265 * t2923 * t21369 * t1268 - 4.0_f64 / 3.0_f64 * t2265 * t4342 * t88149 + 2.0_f64 / 9.0_f64 * t2265 * t4334 * t88153 + 4.0_f64 * t2265 * t2923 * t4969 * t5468 + 2.0_f64 * t2265 * t43164 * t4965 * t5457 + 8.0_f64 / 3.0_f64 * t2265 * t10845 * t21355 * t1268 - 8.0_f64 * t2265 * t2923 * t21362 * t1268 - 12.0_f64 * t2265 * t10864 * t4969 * t5457;
    t91387
}
