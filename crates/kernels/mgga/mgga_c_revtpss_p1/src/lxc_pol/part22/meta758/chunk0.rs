//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2837/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2837<F: Float>(t11626: F, t358: F, t3145: F, t3153: F, t3154: F, t11988: F, t3188: F, t11263: F, t3124: F, t11262: F, t3150: F, t3156: F) -> (F, F, F, F, F, F, F) {
    let t42862 = F::new(1.0) / t11626 / t358;
    let t42864 = t3145 * t3145;
    let t42865 = F::new(1.0) / t42864;
    let t42871 = t3153 * t3153;
    let t42872 = t3154 * t3154;
    let t42907 = t3188 * t11988;
    let t42926 = t3124 * t11263;
    let t42929 = t3150 * t11262 * t3156;
    (t42862, t42865, t42871, t42872, t42907, t42926, t42929)
}
