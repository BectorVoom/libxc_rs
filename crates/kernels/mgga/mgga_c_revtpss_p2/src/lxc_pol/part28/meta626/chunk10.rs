//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2245/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2245<F: Float>(t1113: F, t4433: F, t892: F, t14749: F, t27763: F, t14767: F, t1711: F, t2408: F, t14468: F, t33: F, t25759: F, t61102: F) -> (F, F, F, F, F, F) {
    let t101029 = t892 * t1113 * t4433;
    let t101032 = t27763 * t14749;
    let t101035 = t27763 * t14767;
    let t101040 = t1711 * t2408;
    let t101051 = t33 * t14468;
    let t101055 = t25759 * t61102;
    (t101029, t101032, t101035, t101040, t101051, t101055)
}
