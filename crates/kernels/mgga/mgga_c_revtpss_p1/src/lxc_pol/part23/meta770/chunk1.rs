//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2572/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2572<F: Float>(t57270: F, t1222: F, t5368: F, t697: F, t3625: F, t44250: F, t5406: F, t3781: F, t5219: F, t5330: F, t12881: F, t5391: F) -> (F, F, F, F, F) {
    let t57271 = t57270 / F::new(162.0);
    let t57273 = t1222 * t697 * t5368;
    let t57274 = t57273 / F::new(432.0);
    let t57331 = t3625 * t44250 * t5406;
    let t57382 = t5219 * t3781 * t5330;
    let t57421 = t5391 * t12881;
    (t57271, t57274, t57331, t57382, t57421)
}
