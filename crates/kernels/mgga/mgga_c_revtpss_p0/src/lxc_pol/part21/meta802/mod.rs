//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta802 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2914;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2915;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta802<F: Float>(t300: F, t52368: F, t15547: F, t3030: F, t3012: F, t1634: F, t52239: F, t15520: F, t3022: F, t52481: F, t52486: F, t52488: F, t52490: F, t52492: F, t52495: F, t52499: F, t3026: F, t11616: F, t4719: F, t11598: F, t11507: F, t15266: F, t11591: F, t4725: F, t15556: F, t11105: F, t11108: F, t1699: F, t3333: F, t41937: F, t5019: F, t5023: F, t52502: F, t52507: F, t52510: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52874, t52876, t52880, t52882, t52883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2914::<F>(t300, t52368, t15547, t3030, t3012, t1634, t52239, t15520, t3022, t52481, t52486, t52488, t52490, t52492, t52495, t52499);
        let (t52885, t52887, t52889, t52897, t52899, t52905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2915::<F>(t15547, t3026, t11616, t4719, t11598, t11507, t300, t15266, t52239, t11591, t4725, t15556, t3022);
        let t52906 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2916::<F>(t11105, t11108, t1699, t3333, t41937, t5019, t5023, t52502, t52507, t52510, t52885, t52887, t52889, t52897, t52899, t52905);
    (t52874, t52876, t52880, t52882, t52883, t52885, t52887, t52889, t52897, t52899, t52905, t52906)
}
