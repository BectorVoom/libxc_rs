//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 649/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk649<F: Float>(t1222: F, t2367: F, t1220: F, t1221: F, t2860: F, t914: F, t3086: F, t496: F, t2850: F, t2856: F, t1188: F, t1223: F, t277: F, t2990: F, t2997: F, t3015: F, t3023: F, t3125: F, t3268: F, t3274: F, t95: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3277 = t2367 * t1222;
    let t3278 = t1220 * t3277;
    let t3280 = t1221 * t2860;
    let t3281 = t914 * t3280;
    let t3284 = t3086 * t496;
    let t3285 = t3284 * t2850;
    let t3286 = t914 * t3285;
    let t3289 = t1221 * t2856;
    let t3290 = t914 * t3289;
    let t3293 = F::cast_from(0.25844881434903430496e-2_f64) * t95 * t277 * t3268 * t1188 - t2997 + t2990 + t3015 + t3023 + t3125 + t3274 * t1223 / F::new(3.0) + t3278 / F::new(9.0) + t1220 * t3281 / F::new(6.0) + F::new(2.0) / F::new(9.0) * t1220 * t3286 - t1220 * t3290 / F::new(3.0);
    (t3277, t3280, t3281, t3284, t3285, t3286, t3289, t3290, t3293)
}
