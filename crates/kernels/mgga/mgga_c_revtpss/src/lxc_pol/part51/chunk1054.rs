//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1054/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1054<F: Float>(t31999: F, t8513: F, t93488: F, t1982: F, t31926: F, t3268: F, t31927: F, t994: F, t120361: F, t11921: F, t247: F, t31920: F, t31921: F) -> (F, F, F, F, F) {
    let t120495 = t8513 * t93488 * t31999;
    let t120507 = t1982 * t31926 * t3268;
    let t120513 = t994 * t31927;
    let t120532 = t994 * t120361;
    let t120538 = t31920 * t247 * t11921 * t31921;
    (t120495, t120507, t120513, t120532, t120538)
}
