//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 858/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk858<F: Float>(t11249: F, t3154: F, t11248: F, t1042: F, t1036: F, t11244: F, t11240: F, t357: F, t246: F, t676: F, t1046: F, t1041: F, t1038: F, t3229: F, t1033: F, t3169: F, t3173: F) -> (F, F, F, F, F, F, F) {
    let t11250 = t11249 * t3154;
    let t11251 = t11248 * t11250;
    let t11252 = t1042 * t11251;
    let t11255 = t1036 * t11244;
    let t11256 = t11240 * t11255;
    let t11257 = t11249 * t357;
    let t11258 = t11248 * t11257;
    let t11259 = t1042 * t11258;
    let t11262 = t246 * t676;
    let t11263 = t11262 * t1046;
    let t11264 = t1041 * t11263;
    let t11266 = t3229 * t1038;
    let t11267 = t1036 * t11266;
    let t11268 = t1033 * t11267;
    let t11271 = t3169 * t3173;
    (t11252, t11256, t11259, t11262, t11264, t11268, t11271)
}
