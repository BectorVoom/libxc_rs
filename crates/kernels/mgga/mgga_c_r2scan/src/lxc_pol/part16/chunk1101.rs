//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1101/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1101<F: Float>(t3446: F, t3453: F, t9066: F, t9069: F, t9072: F, t10648: F, t10649: F, t11582: F, t2768: F, t3033: F, t58: F, t597: F, t37524: F, t37528: F, t43735: F, t43739: F, t43742: F, t43747: F, t43750: F, t43752: F, t43754: F, t43756: F) -> (F,) {
    let t43892 = t3446 * t3453 * t9066;
    let t43895 = t3446 * t3453 * t9069;
    let t43898 = t3446 * t3453 * t9072;
    let t43902 = t10648 * t10649 * t11582 * t2768;
    let t43907 = t10648 * t10649 * t58 * t3033 * t597;
    let t43909 = -0.36021158228745895953e-3 * t43892 - 0.72042316457491791906e-3 * t43895 - 0.72042316457491791906e-3 * t43898 + t43735 + 0.72042316457491791906e-3 * t43902 + 0.36021158228745895953e-3 * t43907 + t43739 + t43742 + t43747 + t43750 + t43752 - t43754 - t43756 + t37524 - t37528;
    (t43909,)
}
