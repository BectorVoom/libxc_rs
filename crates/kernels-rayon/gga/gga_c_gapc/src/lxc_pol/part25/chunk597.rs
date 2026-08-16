//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 597/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk597(t3439: f64, t829: f64, t3438: f64, t311: f64, t896: f64, t315: f64, t3171: f64, t3398: f64, t3400: f64, t3409: f64, t3416: f64, t3419: f64, t3422: f64, t3425: f64, t3428: f64, t3432: f64, t3435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3440 = t829 * t3439;
    let t3441 = t3438 * t3440;
    let t3443 = t311 * t896;
    let t3444 = t3171 * t315;
    let t3445 = t3443 * t3444;
    let t3447 = -0.13900948042322754167e-2_f64 * t3398 + 0.54106179813099907243e-4_f64 * t3400 + 0.42205124476153752644e-7_f64 * t3409 + 0.84410248952307505288e-7_f64 * t3416 - 0.37941869869339964455e-7_f64 * t3419 + 0.67460644627686456803e-7_f64 * t3422 - 0.86880925264517213544e-4_f64 * t3425 - 0.86880925264517213544e-4_f64 * t3428 + 0.7240077105376434462e-6_f64 * t3432 - 0.11594181388521408695e-4_f64 * t3435 + 0.14068374825384584215e-8_f64 * t3441 - 0.34752370105806885418e-4_f64 * t3445;
    (t3440, t3441, t3443, t3444, t3445, t3447)
}
