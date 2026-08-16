//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2092/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2092<F: Float>(t17288: F, t2142: F, t5216: F, t1209: F, t2143: F, t26852: F, t5378: F, t29083: F, t3636: F, t1234: F, t29082: F, t17620: F, t26870: F) -> (F, F, F, F, F, F, F) {
    let t104521 = t17288 * t2142;
    let t104524 = t5216 * t2142;
    let t104549 = t1209 * t2143;
    let t104624 = F::cast_from(0.3811023832717309953e-3_f64) * t26852 * t5378;
    let t104626 = F::cast_from(0.20325460441158986416e-2_f64) * t29083 * t3636;
    let t104636 = t1234 * t29082;
    let t104640 = F::cast_from(0.57165357490759649296e-3_f64) * t26870 * t17620;
    (t104521, t104524, t104549, t104624, t104626, t104636, t104640)
}
