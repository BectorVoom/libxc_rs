//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 608/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk608<F: Float>(t10693: F, t10696: F, t10700: F, t10703: F, t10706: F, t10708: F, t10710: F, t10716: F, t10720: F, t10733: F, t10735: F, t10739: F, t9754: F, t9762: F, t12236: F, t1843: F) -> (F, F) {
    let t12317 = -t10693 + t10696 - t10700 + t10703 + t10706 - t10708 + t10710 + t10716 + t10720 + t9754 + t9762 + t10733 + t10735 - t10739;
    let t12318 = t1843 * t12236;
    (t12317, t12318)
}
