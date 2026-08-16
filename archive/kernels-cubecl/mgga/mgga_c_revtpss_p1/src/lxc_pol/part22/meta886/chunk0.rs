//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3072/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3072<F: Float>(t15199: F, t698: F, t2852: F, t373: F, t2439: F, t4628: F, t1606: F, t9303: F, t11387: F, t4631: F, t15513: F, t914: F) -> (F, F, F, F, F, F) {
    let t52065 = t698 * t15199;
    let t52110 = t373 * t2852;
    let t52126 = t2439 * t4628;
    let t52128 = t9303 * t1606;
    let t52163 = t4631 * t11387;
    let t52214 = t15513 * t914;
    (t52065, t52110, t52126, t52128, t52163, t52214)
}
