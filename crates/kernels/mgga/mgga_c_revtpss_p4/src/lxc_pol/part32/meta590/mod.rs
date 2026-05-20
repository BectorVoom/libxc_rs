//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1920;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta590<F: Float>(t28399: F, t686: F, t72: F, t7058: F, t103000: F, t93371: F, t25410: F, t8011: F, t93240: F, t7064: F, t28447: F, t689: F, t887: F, t26485: F, t99463: F, t102986: F, t25387: F, t1580: F, t2439: F, t26434: F, t2453: F, t2458: F, t7998: F, t41040: F, t685: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t103119, t103122, t103130, t103136, t103140) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1920::<F>(t28399, t686, t72, t7058, t103000, t93371, t25410, t8011, t93240, t7064, t28447, t689, t887);
        let (t103142, t103156, t103158, t103161, t103181) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1921::<F>(t26485, t99463, t102986, t25387, t1580, t2439, t26434, t2453, t2458, t7998, t41040, t685);
    (t103119, t103122, t103130, t103136, t103140, t103142, t103156, t103158, t103161, t103181)
}
