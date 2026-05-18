//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 964/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk964<F: Float>(t496: F, t8545: F, t492: F, t490: F, t1210: F, t2839: F, t1188: F, t1220: F, t1223: F, t277: F, t3274: F, t3281: F, t3286: F, t8410: F, t8417: F, t8422: F, t8431: F, t8436: F, t8444: F, t9221: F, t95: F) -> (F, F) {
    let t9226 = t8545 * t496;
    let t9227 = t492 * t9226;
    let t9229 = F::new(5.0) / F::new(27.0) * t490 * t9227;
    let t9230 = t1210 * t2839;
    let t9232 = t8410 * t1223 / F::new(2.0) - F::new(4.0) / F::new(3.0) * t1220 * t8417 + t1220 * t8422 + F::new(14.0) / F::new(27.0) * t1220 * t8431 + t1220 * t8436 / F::new(6.0) + t3274 * t3281 / F::new(2.0) + F::new(2.0) / F::new(3.0) * t3274 * t3286 + t8444 / F::new(6.0) + F::new(0.25844881434903430496e-2) * t95 * t277 * t9221 * t1188 + t9229 - t9230 / F::new(3.0);
    (t9227, t9232)
}
