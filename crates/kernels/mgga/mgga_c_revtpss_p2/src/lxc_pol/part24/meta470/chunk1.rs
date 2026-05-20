//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1449/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1449<F: Float>(t10716: F, t18423: F, t18415: F, t9775: F, t18410: F, t10995: F, t18804: F, t2470: F, t18725: F, t2798: F, t10069: F, t18738: F) -> (F, F, F, F, F, F) {
    let t62431 = t10716 * t18423;
    let t62443 = t9775 * t18415;
    let t62445 = t9775 * t18410;
    let t62528 = t10995 * t18804 * t2470;
    let t62633 = t2798 * t18725 * t2470;
    let t62649 = t10069 * t18738;
    (t62431, t62443, t62445, t62528, t62633, t62649)
}
