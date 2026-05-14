//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 947/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk947<F: Float>(t252: F, t2769: F, t786: F, t2435: F, t2448: F, t2440: F, t887: F, t2439: F, t866: F, t225: F, t2461: F, t2471: F, t788: F, t9288: F, t787: F, t2453: F, t861: F) -> (F, F, F, F, F, F, F) {
    let t10994 = t252 * t2769;
    let t10995 = t786 * t10994;
    let t11000 = t2435 * t2448;
    let t11003 = t2440 * t887;
    let t11004 = t2439 * t11003;
    let t11006 = t866 * t866;
    let t11007 = 1.0 / t11006;
    let t11008 = t225 * t11007;
    let t11013 = t2461 * t2471;
    let t11015 = t788 * t9288;
    let t11017 = 0.30356481678079769392e-1 * t787 * t11015;
    let t11018 = t2453 * t861;
    (t10995, t11000, t11004, t11008, t11013, t11017, t11018)
}
