//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2329/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2329<F: Float>(t10638: F, t231: F, t268: F, t2798: F, t675: F, t2645: F, t837: F, t2782: F, t2797: F, t10115: F, t883: F, t2482: F, t2811: F, t39588: F, t686: F, t72: F) -> (F, F, F, F, F) {
    let t39617 = t2798 * t268 * t675 * t10638 * t231;
    let t39620 = t837 * t2645;
    let t39622 = t2782 * t2797 * t39620;
    let t39624 = t10115 * t883;
    let t39629 = t2482 * t2811 * t72 * t686 * t39588;
    (t39617, t39620, t39622, t39624, t39629)
}
