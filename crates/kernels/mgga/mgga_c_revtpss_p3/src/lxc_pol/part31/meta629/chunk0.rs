//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2083/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2083<F: Float>(t25526: F, t4820: F, t15769: F, t25522: F, t15687: F, t25515: F, t3317: F, t25525: F, t4878: F, t27450: F, t3173: F, t16035: F, t25580: F) -> (F, F, F, F, F, F, F) {
    let t100048 = t25526 * t4820;
    let t100051 = F::cast_from(0.3811023832717309953e-3_f64) * t25522 * t15769;
    let t100054 = t25515 * t15687;
    let t100055 = t3317 * t100054;
    let t100074 = t4878 * t25525;
    let t100078 = F::cast_from(0.57165357490759649296e-3_f64) * t27450 * t3173;
    let t100092 = F::cast_from(0.57165357490759649296e-3_f64) * t25580 * t16035;
    (t100048, t100051, t100054, t100055, t100074, t100078, t100092)
}
