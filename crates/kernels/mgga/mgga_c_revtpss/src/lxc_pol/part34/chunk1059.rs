//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1059/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1059<F: Float>(t6856: F, t7271: F, t6876: F, t7264: F, t26017: F, t6850: F, t26028: F, t6871: F, t6884: F, t7252: F, t25983: F, t6864: F, t26003: F, t26011: F, t26013: F, t26022: F, t27921: F, t27953: F, t28873: F, t28874: F, t28885: F) -> (F, F, F, F, F) {
    let t30039 = t7271 * t6856;
    let t30041 = t7264 * t6876;
    let t30043 = t26017 * t6850;
    let t30045 = t26028 * t6871;
    let t30048 = t7252 * t6884;
    let t30050 = t25983 * t6864;
    let t30054 = t26003 - t26011 - t30048 / 48.0 + t28885 + 0.85748036236139473944e-3 * t30050 + t26013 + t26022 - 0.50820002809285328226e-4 * t27953 + t28873 + t28874 + 0.40015750243531754508e-2 * t27921;
    (t30039, t30041, t30043, t30045, t30054)
}
