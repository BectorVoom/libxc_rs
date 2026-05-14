//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 342/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk342<F: Float>(t1954: F, t741: F, t1932: F, t1938: F, t1942: F, t1946: F, t1951: F) -> (F, F) {
    let t1955 = t741 * t1954;
    let t1957 = t1932 / 16.0 - t1938 / 16.0 + t1942 / 24.0 - t1946 / 256.0 + t1951 / 256.0 - t1955 / 192.0;
    (t1955, t1957)
}
