//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1061/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1061<F: Float>(t11981: F, t11984: F, t11988: F, t11992: F, t11995: F, t11998: F, t10099: F, t3568: F, t1096: F, t3622: F, t2469: F, t3832: F, t972: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12269 = F::cast_from(0.35848176214430067276e-9_f64) * t11981;
    let t12270 = F::cast_from(0.33147827249531850013e-7_f64) * t11984;
    let t12271 = F::cast_from(0.34752370105806885418e-3_f64) * t11988;
    let t12272 = F::cast_from(0.4637672555408563478e-4_f64) * t11992;
    let t12273 = F::cast_from(0.4637672555408563478e-4_f64) * t11995;
    let t12274 = F::cast_from(0.38647271295071362317e-6_f64) * t11998;
    let t12281 = F::cast_from(2.0_f64) * t10099 * t3568;
    let t12285 = t3622 * t1096;
    let t12287 = F::cast_from(2.0_f64) * t2469 * t12285;
    let t12288 = t3832 * t972;
    (t12269, t12270, t12271, t12272, t12273, t12274, t12281, t12285, t12287, t12288)
}
