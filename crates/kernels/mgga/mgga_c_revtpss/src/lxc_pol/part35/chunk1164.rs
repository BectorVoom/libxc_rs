//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1164/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1164<F: Float>(t30261: F, t689: F, t25904: F, t25899: F, t1358: F, t212: F, t30247: F, t1904: F, t28824: F, t109407: F, t7289: F, t27884: F, t28845: F) -> (F, F, F, F, F, F) {
    let t109457 = t30261 * t689;
    let t109458 = t25904 * t109457;
    let t109460 = t25899 * t109457;
    let t109488 = t689 * t212 * t30247 * t1358;
    let t109505 = t689 * t28824 * t1904;
    let t109512 = t7289 * t109407;
    let t109514 = t27884 * t28845;
    (t109458, t109460, t109488, t109505, t109512, t109514)
}
