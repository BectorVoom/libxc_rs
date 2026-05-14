//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1148/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1148<F: Float>(t11735: F, t1968: F, t11772: F, t25515: F, t3114: F, t11923: F, t25580: F, t11240: F, t11244: F, t7120: F, t12020: F, t7121: F, t3223: F, t7131: F, t1033: F, t11266: F) -> (F, F, F, F, F, F, F) {
    let t93750 = 5.0 / 1296.0 * t1968 * t11735;
    let t93751 = t25515 * t11772;
    let t93752 = t3114 * t93751;
    let t93755 = t25580 * t11923;
    let t93758 = t11240 * t7120 * t11244;
    let t93761 = t12020 * t7121;
    let t93764 = t3223 * t7131;
    let t93774 = t1033 * t7120 * t11266;
    (t93750, t93752, t93755, t93758, t93761, t93764, t93774)
}
