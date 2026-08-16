//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1497/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1497<F: Float>(t23384: F, t689: F, t779: F, t14987: F, t18797: F, t23388: F, t786: F, t789: F, t23414: F, t23413: F, t41070: F, t686: F, t72: F) -> (F, F, F, F, F) {
    let t75950 = t689 * t779 * t23384;
    let t75956 = t14987 * t18797;
    let t75961 = t786 * t23388 * t789;
    let t75974 = t689 * t779 * t23414;
    let t75978 = t41070 * t23413 * t72 * t686;
    (t75950, t75956, t75961, t75974, t75978)
}
