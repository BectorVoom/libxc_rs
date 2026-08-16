//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2781/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2781<F: Float>(t22236: F, t808: F, t9736: F, t6884: F, t9741: F, t14104: F, t47856: F, t13729: F, t2782: F, t556: F, t5774: F, t2439: F, t3895: F, t6896: F) -> (F, F, F, F, F) {
    let t74714 = t9736 * t808 * t22236;
    let t74717 = t9741 * t6884;
    let t74733 = t47856 * t14104;
    let t74744 = t2782 * t556 * t13729 * t5774;
    let t74757 = t2439 * t3895 * t6896;
    (t74714, t74717, t74733, t74744, t74757)
}
