//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 855/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk855<F: Float>(t151: F, t16381: F, t16401: F, t2126: F, t16405: F, t16385: F, t16389: F, t13392: F, t16382: F, t16386: F, t16390: F, t16402: F, t16406: F, t2124: F, t2168: F, t3467: F, t3501: F) -> (F, F, F, F, F, F) {
    let t16534 = t151 * t16381;
    let t16537 = t2126 * t16401;
    let t16540 = t2126 * t16405;
    let t16543 = t2126 * t16385;
    let t16546 = t151 * t16389;
    let t16549 = F::cast_from(0.2115989587251296286e0_f64) * t13392 + F::cast_from(0.18137053605011111023e0_f64) * t2168 * t16406 - F::cast_from(0.45342634012527777558e-1_f64) * t2168 * t16382 + F::cast_from(0.18137053605011111023e0_f64) * t2168 * t16402 - F::cast_from(0.5441116081503333307e0_f64) * t3501 * t16386 + F::cast_from(0.13602790203758333267e0_f64) * t3501 * t16390 - F::cast_from(0.26079484469366273564e0_f64) * t2124 * t16534 + F::cast_from(0.52158968938732547127e0_f64) * t2124 * t16537 + F::cast_from(0.52158968938732547127e0_f64) * t2124 * t16540 - F::cast_from(0.10431793787746509425e1_f64) * t3467 * t16543 + F::cast_from(0.52158968938732547127e0_f64) * t3467 * t16546;
    (t16534, t16537, t16540, t16543, t16546, t16549)
}
