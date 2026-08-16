//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 756/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk756<F: Float>(t14255: F, t73692: F, t3148: F, t3151: F, t38471: F, t446: F, t511: F, t558: F, t14117: F, t68448: F, t68455: F, t9205: F) -> (F, F, F, F, F) {
    let t73693 = t73692 * t14255;
    let t73696 = t38471 * t3148 * t3151;
    let t73699 = t511 * t558 * t446;
    let t73701 = t68448 * t14117 * t73699;
    let t73704 = t68455 * t14117 * t9205;
    (t73693, t73696, t73699, t73701, t73704)
}
