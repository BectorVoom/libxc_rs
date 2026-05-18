//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1303/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1303<F: Float>(t1049: F, t24906: F, t10099: F, t12002: F, t2469: F, t2470: F, t3268: F, t338: F, t3449: F, t35757: F, t35803: F, t35845: F, t35887: F, t35927: F, t35964: F, t36008: F, t36046: F, t36055: F, t36058: F, t36067: F, t36068: F, t36072: F, t36074: F, t36078: F, t3795: F, t7063: F, t9378: F, t972: F) -> (F, F) {
    let t36080 = F::new(2.0) * t24906 * t1049;
    let t36081 = (t35757 + t35803 + t35845 + t35887 + t35927 + t35964 + t36008 + t36046) * t338 - F::new(24.0) * t7063 * t3268 * t3449 + t36055 - t36058 - F::new(6.0) * t7063 * t3795 * t2470 + F::new(4.0) * t2469 * t12002 * t972 + t36067 + F::new(4.0) * t2469 * t36068 - t36072 - t36074 + F::new(8.0) * t10099 * t9378 - t36078 + t36080;
    (t36080, t36081)
}
