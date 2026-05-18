//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 973/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk973<F: Float>(t14717: F, t3338: F, t5046: F, t3334: F, t5083: F, t1797: F, t3429: F, t1200: F, t5169: F, t14595: F, t3438: F, t3437: F) -> (F, F, F, F, F, F) {
    let t14726 = t3338 * t14717;
    let t14727 = t5046 * t14726;
    let t14729 = t5083 * t3334;
    let t14731 = t1797 * t3429;
    let t14733 = t5169 * t1200;
    let t14735 = t3438 * t14595;
    let t14736 = t3437 * t14735;
    (t14727, t14729, t14731, t14733, t14735, t14736)
}
