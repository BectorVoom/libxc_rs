//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1261/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1261<F: Float>(t225: F, t9990: F, t213: F, t10605: F, t162: F, t10439: F, t2394: F, t262: F, t10867: F, t10871: F, t2722: F, t73: F, t830: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14192 = t225 * t9990;
    let t14193 = t213 * t14192;
    let t14325 = t10605 * t162;
    let t14330 = t10439 * t162;
    let t14375 = t2394 * t262;
    let t14545 = t225 * t10867;
    let t14546 = t213 * t14545;
    let t14547 = t10871 * t2722;
    let t14643 = t830 * t73;
    (t14192, t14193, t14325, t14330, t14375, t14545, t14546, t14547, t14643)
}
