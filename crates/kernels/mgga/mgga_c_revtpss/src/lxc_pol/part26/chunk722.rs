//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 722/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk722<F: Float>(t10006: F, t9755: F, t9824: F, t9928: F, t225: F, t1419: F, t4086: F, t786: F, t4104: F, t268: F, t4056: F, t543: F, t675: F, t4101: F, t555: F, t5744: F) -> (F, F, F, F, F) {
    let t10008 = t9755 + t9824 + t9928 + t10006;
    let t10009 = t10008 * t225;
    let t10013 = t4086 * t1419;
    let t10014 = t786 * t10013;
    let t10015 = t10014 * t4104;
    let t10019 = t268 * t675 * t4056 * t543;
    let t10020 = t4101 * t10019;
    let t10022 = t5744 * t555;
    (t10008, t10009, t10015, t10020, t10022)
}
