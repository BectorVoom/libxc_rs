//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1404/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1404<F: Float>(t34800: F, t10241: F, t9448: F, t15482: F, t20560: F, t9439: F, t20555: F, t10543: F, t1407: F, t1429: F, t2365: F, t2366: F, t25729: F) -> (F, F, F, F, F) {
    let t34801 = F::cast_from(0.89376224879626066674e-1_f64) * t34800;
    let t34814 = t9448 * t10241;
    let t34817 = F::cast_from(0.5680433474654925878e0_f64) * t20560 * t15482 * t34814;
    let t34818 = t9439 * t10241;
    let t34821 = F::cast_from(0.22721733898619703511e0_f64) * t20555 * t15482 * t34818;
    let t34822 = t1407 * t10543;
    let t34823 = F::cast_from(0.51123901271894332902e0_f64) * t34822;
    let t34826 = t1429 * t2365 * t2366 * t25729;
    (t34801, t34817, t34821, t34823, t34826)
}
