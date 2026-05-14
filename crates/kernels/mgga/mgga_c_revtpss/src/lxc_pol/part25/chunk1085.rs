//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1085/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1085<F: Float>(t1983: F, t27639: F, t4975: F, t988: F, t1096: F, t27638: F, t3143: F, t33: F, t892: F, t11064: F, t1955: F, t7283: F, t13846: F, t1941: F, t241: F, t25981: F, t820: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27640 = t1983 * t27639;
    let t27652 = t4975 * t988;
    let t27664 = t4975 * t1096;
    let t27668 = t27638 * t3143;
    let t27669 = t1983 * t27668;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27868 = t1955 * t7283;
    let t27932 = t1941 * t13846;
    let t27940 = t820 * t25981 * t241;
    (t27640, t27652, t27664, t27668, t27669, t27763, t27799, t27868, t27932, t27940)
}
