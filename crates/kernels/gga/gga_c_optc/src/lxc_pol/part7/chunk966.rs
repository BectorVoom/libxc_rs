//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 966/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk966<F: Float>(t3274: F, t3277: F, t1222: F, t7274: F, t1220: F, t1186: F, t277: F, t2911: F, t3268: F, t3290: F, t3980: F, t4281: F, t8552: F, t8695: F, t9234: F, t9237: F, t9241: F, t9244: F, t9251: F, t9254: F, t95: F) -> (F, F) {
    let t9258 = t3274 * t3277;
    let t9260 = t7274 * t1222;
    let t9261 = t1220 * t9260;
    let t9263 = F::new(2.0) / F::new(9.0) * t9234 - t9237 / F::new(3.0) - t3274 * t3290 - t4281 * t9241 + F::new(2.0) / F::new(3.0) * t4281 * t9244 - F::cast_from(0.77534644304710291488e-2_f64) * t3980 * t1186 * t2911 * t3268 + t8552 + F::cast_from(0.51689762869806860992e-2_f64) * t95 * t277 * t9251 * t9254 + t9258 / F::new(3.0) - t9261 / F::new(9.0) - t8695;
    (t9260, t9263)
}
