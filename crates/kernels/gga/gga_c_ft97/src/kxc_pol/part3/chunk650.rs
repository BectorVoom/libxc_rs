//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 650/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk650<F: Float>(t1186: F, t89: F, t9733: F, t13730: F, t4044: F, t1471: F, t4092: F, t1701: F, t213: F, t2725: F, t6: F, t285: F, t1196: F, t2724: F, t1200: F, t4088: F, t816: F) -> (F, F, F, F, F, F, F, F) {
    let t14715 = t89 * t9733 * t1186;
    let t14718 = t89 * t13730 * t4044;
    let t14721 = t4092 * t1471;
    let t14722 = t1701 * t213;
    let t14728 = t2725 * t6;
    let t14729 = t285 * t14728;
    let t14738 = t2724 * t1196;
    let t14742 = t1200 * t14728;
    let t14752 = t816 * t4088;
    (t14715, t14718, t14721, t14722, t14729, t14738, t14742, t14752)
}
