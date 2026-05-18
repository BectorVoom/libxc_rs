//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 806/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk806<F: Float>(t13105: F, t381: F, t1795: F, t3225: F, t3436: F, t5025: F, t10513: F, t284: F, t41: F, t9545: F, t9588: F, t1094: F, t5163: F) -> (F, F, F, F, F, F, F) {
    let t14721 = t13105 * t381;
    let t14781 = t1795 * t3225;
    let t14785 = t5025 * t3436;
    let t14832 = t10513 * t284;
    let t14838 = t41 * t9545;
    let t14849 = t9588 * t3436;
    let t14874 = t5163 * t1094;
    (t14721, t14781, t14785, t14832, t14838, t14849, t14874)
}
