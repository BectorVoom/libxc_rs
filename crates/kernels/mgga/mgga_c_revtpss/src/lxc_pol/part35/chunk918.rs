//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 918/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk918<F: Float>(t1211: F, t24713: F, t1828: F, t6587: F, t1277: F, t6573: F, t24543: F, t487: F, t13143: F, t24864: F, t489: F, t1287: F, t1794: F, t6695: F, t5486: F, t1811: F, t6622: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24892 = t1211 * t24713;
    let t24899 = t6587 * t1828;
    let t24900 = t1277 * t24899;
    let t24906 = t1277 * t6573 * t1828;
    let t24911 = t487 * t24543;
    let t24912 = t24911 * t13143;
    let t24915 = t489 * t24864;
    let t24919 = t6695 * t1794 * t1287;
    let t24922 = t5486 * t6573;
    let t24928 = t1811 * t6622 * t1287;
    (t24892, t24900, t24906, t24911, t24912, t24915, t24919, t24922, t24928)
}
