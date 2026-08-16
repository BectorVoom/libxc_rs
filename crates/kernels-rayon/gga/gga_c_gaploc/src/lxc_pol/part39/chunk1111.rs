//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1111/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1111(t2679: f64, t3726: f64, t9796: f64, t12240: f64, t2617: f64, t7810: f64, t38961: f64, t935: f64, t1457: f64, t2103: f64, t43444: f64, t43447: f64, t43448: f64, t43449: f64, t43450: f64, t43455: f64, t43456: f64, t43458: f64, t43462: f64) -> (f64, f64) {
    let t47212 = t9796 * t3726 * t2679;
    let t47215 = t7810 * t12240 * t2617;
    let t47220 = t38961 * t935;
    let t47222 = t2103 * t1457 * t47220;
    let t47224 = -0.38342925953920749676e0_f64 * t47212 - 0.19171462976960374838e0_f64 * t47215 - t43444 - t43447 + t43448 - t43449 + t43450 - t43455 + 0.35750489951850426669e0_f64 * t43456 + 0.14896037479937677779e-1_f64 * t43458 + 0.14896037479937677779e-1_f64 * t43462 + 0.71500979903700853338e0_f64 * t47222;
    (t47220, t47224)
}
