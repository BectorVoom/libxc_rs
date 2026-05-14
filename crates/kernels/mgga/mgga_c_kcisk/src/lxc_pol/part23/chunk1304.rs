//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1304/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1304<F: Float>(t32628: F, t32652: F, t1129: F, t15681: F, t15682: F, t397: F, t9379: F, t32633: F, t110994: F, t3373: F, t9368: F, t43680: F, t9364: F, t15705: F, t32646: F, t32643: F, t32664: F) -> (F, F, F, F, F, F, F) {
    let t111127 = t32652 * t32628;
    let t111132 = t9379 * t397 * t15681 * t1129 * t15682;
    let t111134 = t32652 * t32633;
    let t111137 = t3373 * t110994 * t9368;
    let t111140 = t43680 * t9364 * t32633;
    let t111143 = t15705 * t32646 * t32633;
    let t111145 = t32664 * t32643;
    (t111127, t111132, t111134, t111137, t111140, t111143, t111145)
}
