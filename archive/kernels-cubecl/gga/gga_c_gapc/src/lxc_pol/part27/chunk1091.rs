//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1091/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1091<F: Float>(t10293: F, t29664: F, t33498: F, t3437: F, t11449: F, t11815: F, t190: F, t2786: F, t33374: F, t7595: F, t15553: F, t15555: F, t33287: F) -> (F, F, F, F) {
    let t33501 = t3437 * t33498 * t10293 * t29664;
    let t33505 = t2786 * t190 * t11449 * t11815;
    let t33507 = t33374 * t7595;
    let t33510 = t15553 * t33287 * t15555;
    (t33501, t33505, t33507, t33510)
}
