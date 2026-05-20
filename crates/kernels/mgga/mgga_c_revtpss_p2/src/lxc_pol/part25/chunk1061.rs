//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1061/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1061<F: Float>(t3555: F, t3754: F, t1248: F, t3153: F, t3588: F, t5464: F, t3566: F, t3568: F, t1287: F, t1269: F, t1284: F, t1209: F) -> (F, F, F, F, F, F) {
    let t12709 = t3555 * t3754;
    let t12712 = t1248 * t3153;
    let t12713 = t5464 * t3588;
    let t12714 = t12712 * t12713;
    let t12717 = t3566 * t3754;
    let t12718 = t3568 * t1248;
    let t12719 = t12718 * t1287;
    let t12722 = t1284 * t1269;
    let t12723 = t1209 * t12722;
    (t12709, t12714, t12717, t12718, t12719, t12723)
}
