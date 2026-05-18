//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1208/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1208<F: Float>(t1389: F, t268: F, t221: F, t9984: F, t10115: F, t555: F, t4146: F, t1353: F, t4144: F, t1448: F, t3829: F, t3889: F) -> (F, F, F, F, F, F, F) {
    let t46808 = t1389 * t268;
    let t47300 = t221 * t9984;
    let t47567 = t10115 * t555;
    let t47671 = t4146 * t4146;
    let t47672 = F::new(1.0) / t47671;
    let t49560 = t4144 * t1353;
    let t49616 = t3829 * t1448;
    let t49630 = t3889 * t1448;
    (t46808, t47300, t47567, t47672, t49560, t49616, t49630)
}
