//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 481/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk481<F: Float>(t2349: F, t531: F, t1589: F, t888: F, t1628: F, t907: F, t590: F, t1407: F, t914: F, t1225: F, t124: F, t1390: F) -> (F, F, F, F, F, F, F) {
    let t2446 = t531 * t2349;
    let t2449 = t1589 * t888;
    let t2452 = t1628 * t907;
    let t2457 = t888 * t590;
    let t2460 = t1407 * t914;
    let t2462 = F::cast_from(1.0_f64) / t1225;
    let t2464 = t2462 * t124 * t1390;
    (t2446, t2449, t2452, t2457, t2460, t2462, t2464)
}
