//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1013/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1013<F: Float>(t2014: F, t32129: F, t6972: F, t8441: F, t8621: F, t1936: F, t25805: F, t28025: F, t6985: F, t7002: F, t648: F, t8453: F) -> (F, F, F, F, F, F) {
    let t32131 = F::cast_from(2.0_f64) * t2014 * t32129;
    let t32151 = t8621 * t8441 * t6972;
    let t32165 = t25805 * t1936;
    let t32167 = t28025 * t1936;
    let t32169 = t6985 * t7002;
    let t32171 = t648 * t8453;
    (t32131, t32151, t32165, t32167, t32169, t32171)
}
