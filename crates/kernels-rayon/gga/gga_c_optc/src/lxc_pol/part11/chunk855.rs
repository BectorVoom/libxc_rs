//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 855/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk855(t151: f64, t16381: f64, t16401: f64, t2126: f64, t16405: f64, t16385: f64, t16389: f64, t13392: f64, t16382: f64, t16386: f64, t16390: f64, t16402: f64, t16406: f64, t2124: f64, t2168: f64, t3467: f64, t3501: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16534 = t151 * t16381;
    let t16537 = t2126 * t16401;
    let t16540 = t2126 * t16405;
    let t16543 = t2126 * t16385;
    let t16546 = t151 * t16389;
    let t16549 = 0.2115989587251296286e0_f64 * t13392 + 0.18137053605011111023e0_f64 * t2168 * t16406 - 0.45342634012527777558e-1_f64 * t2168 * t16382 + 0.18137053605011111023e0_f64 * t2168 * t16402 - 0.5441116081503333307e0_f64 * t3501 * t16386 + 0.13602790203758333267e0_f64 * t3501 * t16390 - 0.26079484469366273564e0_f64 * t2124 * t16534 + 0.52158968938732547127e0_f64 * t2124 * t16537 + 0.52158968938732547127e0_f64 * t2124 * t16540 - 0.10431793787746509425e1_f64 * t3467 * t16543 + 0.52158968938732547127e0_f64 * t3467 * t16546;
    (t16534, t16537, t16540, t16543, t16546, t16549)
}
