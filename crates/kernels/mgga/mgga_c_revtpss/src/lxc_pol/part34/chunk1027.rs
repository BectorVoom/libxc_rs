//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1027/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1027<F: Float>(t1035: F, t7810: F, t1982: F, t27418: F, t342: F, t1678: F, t3140: F, t1078: F, t11239: F, t1983: F, t1668: F, t1976: F, t3153: F, t994: F, t3143: F, t1647: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t27604 = t1035 * t7810;
    let t27609 = t1982 * t27418;
    let t27616 = t342 * t7810;
    let t27619 = t1678 * t3140;
    let t27621 = t1982 * t27619 * t1078;
    let t27638 = t11239 * t1078;
    let t27639 = t27638 * t1035;
    let t27640 = t1983 * t27639;
    let t27641 = t1976 * t1668;
    let t27642 = t27641 * t3153;
    let t27661 = t994 * t27418;
    let t27668 = t27638 * t3143;
    let t27669 = t1983 * t27668;
    let t27699 = t1647 * t1976;
    (t27604, t27609, t27616, t27621, t27639, t27640, t27642, t27661, t27668, t27669, t27699)
}
