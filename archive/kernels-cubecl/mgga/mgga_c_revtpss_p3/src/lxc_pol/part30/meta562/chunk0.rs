//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2007/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2007<F: Float>(t857: F, t93066: F, t25222: F, t2656: F, t2482: F, t596: F, t7043: F, t2677: F, t10741: F, t25234: F, t10709: F, t25227: F, t2661: F) -> (F, F, F, F, F, F) {
    let t93067 = t93066 * t857;
    let t93069 = t25222 * t2656;
    let t93072 = t2482 * t7043 * t596;
    let t93073 = t93072 * t2677;
    let t93077 = t25234 * t10741;
    let t93080 = t2661 * t25227 * t10709;
    (t93067, t93069, t93072, t93073, t93077, t93080)
}
