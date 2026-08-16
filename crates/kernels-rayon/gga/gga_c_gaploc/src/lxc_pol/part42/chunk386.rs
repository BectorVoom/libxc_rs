//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 386/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk386(t3025: f64, t3470: f64, t1022: f64, t2610: f64, t2365: f64, t2033: f64, t1457: f64, t3447: f64, t2103: f64, t2949: f64, t935: f64, t1445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3472 = 0.10725146985555128001e1_f64 * t3025 * t3470;
    let t3473 = t2610 * t1022;
    let t3474 = t2365 * t3473;
    let t3475 = t2033 * t3474;
    let t3476 = 0.14896037479937677779e-1_f64 * t3475;
    let t3477 = t1457 * t3447;
    let t3479 = 0.71500979903700853338e0_f64 * t2103 * t3477;
    let t3483 = t2949 * t935;
    let t3484 = t1445 * t3483;
    (t3472, t3473, t3474, t3475, t3476, t3477, t3479, t3483, t3484)
}
