//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1263/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1263<F: Float>(t13781: F, t3222: F, t3306: F, t3972: F, t14657: F, t50891: F, t1114: F, t51916: F, t51919: F, t13888: F, t2408: F, t51505: F, t51507: F, t53526: F, t53529: F, t53531: F, t53537: F, t53542: F, t53544: F, t53546: F, t53549: F, t53553: F, t8764: F, t9283: F) -> F {
    let t53562 = t3972 * t13781 * t3306 * param_a_c * t3222;
    let t53564 = t14657 * t50891;
    let t53566 = t1114 * t51916;
    let t53567 = t53566 * t51919;
    let t53569 = F::new(5.0) / F::new(384.0) * t53526 + t53529 / F::new(768.0) + t53531 / F::new(24.0) - F::new(7.0) / F::new(288.0) * t51505 - F::new(7.0) / F::new(2304.0) * t51507 - t53537 / F::new(3072.0) + t53542 / F::new(1536.0) - t53544 - t53546 - F::new(5.0) / F::new(768.0) * t53549 + t53553 / F::new(768.0) - t2408 * t9283 * t13888 * t8764 / F::new(24.0) - t53562 / F::new(768.0) - t53564 / F::new(48.0) + t53567 / F::new(48.0);
    t53569
}
