//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1312/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1312<F: Float>(t15884: F, t3238: F, t11683: F, t11687: F, t23343: F, t11675: F, t24195: F, t11270: F, t268: F, t190: F, t23608: F, t24110: F) -> (F, F, F, F, F) {
    let t35722 = t3238 * t15884;
    let t35725 = t11687 * t11683 * t23343;
    let t35727 = t11675 * t24195;
    let t35729 = t11270 * t268;
    let t35732 = t35729 * t23608 * t190 * t24110;
    (t35722, t35725, t35727, t35729, t35732)
}
