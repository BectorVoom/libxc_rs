//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 782/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk782<F: Float>(t4251: F, t950: F, t2496: F, t4247: F, t957: F, t1392: F, t238: F, t242: F, t343: F, t4234: F, t2493: F, t2503: F, t3461: F, t3503: F, t4236: F, t4248: F) -> (F, F, F, F, F, F, F, F) {
    let t4252 = t950 * t4251;
    let t4256 = t2496 * t4247;
    let t4258 = t957 * t4251;
    let t4261 = t1392 * t1392;
    let t4263 = t238 * t242 * t4261;
    let t4265 = t343 * t4234;
    let t4267 = t238 * t242 * t4265;
    let t4269 = -F::cast_from(0.9494625e0_f64) * t4248 + F::cast_from(0.1898925e1_f64) * t4252 + t2493 - F::cast_from(0.59793333333333333334e0_f64) * t3461 + F::cast_from(0.8969e0_f64) * t4236 + F::cast_from(0.15358125e0_f64) * t4256 + F::cast_from(0.3071625e0_f64) * t4258 + t2503 - F::cast_from(0.32862666666666666666e0_f64) * t3503 + F::cast_from(0.24647e0_f64) * t4263 + F::cast_from(0.24647e0_f64) * t4267;
    (t4252, t4256, t4258, t4261, t4263, t4265, t4267, t4269)
}
