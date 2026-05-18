//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 938/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk938<F: Float>(t1038: F, t3229: F, t1036: F, t1033: F, t3169: F, t3173: F, t3140: F, t989: F, t3149: F, t3160: F, t2862: F, t3128: F) -> (F, F, F, F, F) {
    let t11266 = t3229 * t1038;
    let t11267 = t1036 * t11266;
    let t11268 = t1033 * t11267;
    let t11271 = t3169 * t3173;
    let t11273 = t989 * t3140;
    let t11274 = t11273 * t3149;
    let t11277 = t11273 * t3160;
    let t11280 = t3128 * t2862;
    (t11268, t11271, t11274, t11277, t11280)
}
