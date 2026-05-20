//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2934/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2934<F: Float>(t14082: F, t3920: F, t14078: F, t2470: F, t3915: F, t13735: F, t2435: F, t10119: F, t14114: F, t10115: F, t1900: F, t14189: F) -> (F, F, F, F, F, F) {
    let t47944 = t14082 * t3920;
    let t47947 = t3915 * t14078 * t2470;
    let t47952 = t2435 * t13735;
    let t47957 = t14114 * t10119;
    let t47961 = t10115 * t1900;
    let t47963 = t2435 * t14189;
    (t47944, t47947, t47952, t47957, t47961, t47963)
}
