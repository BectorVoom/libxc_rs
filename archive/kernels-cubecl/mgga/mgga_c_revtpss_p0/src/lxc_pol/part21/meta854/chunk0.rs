//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3223/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3223<F: Float>(t12050: F, t17710: F, t17191: F, t3555: F, t1269: F, t13147: F, t460: F, t1209: F, t21455: F, t5219: F, t5477: F, t17288: F, t3754: F) -> (F, F, F, F, F, F) {
    let t59650 = t17710 * t12050;
    let t59657 = t3555 * t17191;
    let t59671 = t460 * t13147 * t1269;
    let t59674 = t1209 * t21455;
    let t59681 = t5219 * t5477;
    let t59686 = t17288 * t3754;
    (t59650, t59657, t59671, t59674, t59681, t59686)
}
