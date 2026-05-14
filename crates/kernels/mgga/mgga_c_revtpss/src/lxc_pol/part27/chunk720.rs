//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 720/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk720<F: Float>(t3: F, t7690: F, t1461: F, t2170: F, t573: F, t7329: F, t7333: F, t7336: F, t2033: F, t4147: F, t587: F, t65: F, t3140: F, t3736: F, t1276: F, t1243: F) -> (F, F, F, F, F, F, F) {
    let t7691 = t3 * t7690;
    let t7696 = param_d * t7690;
    let t7700 = 3.0 * t1461 * t2170 + t573 * t7696 + t7329 + t7333 + t7336;
    let t8717 = t4147 * t2033;
    let t8779 = 1.0 / t65 / t587;
    let t8939 = t3140 * t3736;
    let t8944 = t3140 * t1276;
    let t8945 = t8944 * t1243;
    (t7691, t7696, t7700, t8717, t8779, t8939, t8945)
}
