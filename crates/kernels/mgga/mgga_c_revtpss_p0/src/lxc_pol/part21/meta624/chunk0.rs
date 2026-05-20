//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2384/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2384<F: Float>(t2661: F, t2662: F, t2749: F, t40378: F, t2430: F, t853: F, t837: F, t836: F, t124: F, t2645: F, t14686: F, t14931: F, t4366: F) -> (F, F, F, F, F, F) {
    let t40553 = t2661 * t2662 * t40378 * t2749;
    let t40555 = t853 * t2430;
    let t40558 = t2661 * t2662 * t40555 * t837;
    let t40560 = t2430 * t836;
    let t40578 = t124 * t2645;
    let t40581 = t14931 * t14686 * t40578 * t4366;
    (t40553, t40555, t40558, t40560, t40578, t40581)
}
