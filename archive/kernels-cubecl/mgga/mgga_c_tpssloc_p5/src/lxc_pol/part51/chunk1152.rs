//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1152/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1152<F: Float>(t1998: F, t6955: F, t214: F, t1985: F, t2314: F, t8326: F, t5113: F, t191: F, t192: F, t6872: F, t3938: F, t671: F) -> (F, F, F, F, F, F, F, F) {
    let t31206 = t1998 * t6955;
    let t31207 = t214 * t31206;
    let t31209 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t31207;
    let t31236 = t2314 * t8326;
    let t31237 = F::cast_from(2.0_f64) * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = F::cast_from(2.0_f64) * t31238;
    let t31246 = t6872 * t191 * t192;
    let t31283 = t3938 * t8326;
    let t31284 = F::cast_from(0.135e2_f64) * t31283;
    let t31285 = t8326 * t671;
    (t31206, t31207, t31209, t31237, t31239, t31246, t31284, t31285)
}
