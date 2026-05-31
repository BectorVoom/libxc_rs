//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 720/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk720<F: Float>(t2247: F, t8435: F, t1925: F, t136: F, t73: F, t74: F, t84: F) -> (F, F, F, F, F) {
    let t8436 = t2247 * t8435;
    let t8437 = t1925 * t1925;
    let t8438 = t8437 * t136;
    let t8440 = F::cast_from(1.0_f64) / t74 / t73;
    let t8441 = t84 * t84;
    (t8436, t8437, t8438, t8440, t8441)
}
