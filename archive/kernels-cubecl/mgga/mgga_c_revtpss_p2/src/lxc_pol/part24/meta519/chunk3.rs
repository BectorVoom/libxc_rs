//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1546/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1546<F: Float>(t24407: F, t3520: F, t24294: F, t698: F, t24288: F, t24291: F, t24274: F, t24271: F, t24312: F, t3390: F, t24297: F, t24323: F, t3435: F) -> (F, F, F, F, F, F, F, F, F) {
    let t81310 = t3520 * t24407;
    let t81425 = t698 * t24294;
    let t81427 = t698 * t24288;
    let t81429 = t698 * t24291;
    let t81491 = t698 * t24274;
    let t81496 = t698 * t24271;
    let t81513 = t3390 * t24312;
    let t81539 = t698 * t24297;
    let t81650 = t24323 * t3435;
    (t81310, t81425, t81427, t81429, t81491, t81496, t81513, t81539, t81650)
}
