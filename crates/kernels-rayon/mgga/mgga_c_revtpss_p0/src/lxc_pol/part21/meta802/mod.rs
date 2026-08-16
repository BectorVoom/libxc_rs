//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta802 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2914;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2915;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2916;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta802(t300: f64, t52368: f64, t15547: f64, t3030: f64, t3012: f64, t1634: f64, t52239: f64, t15520: f64, t3022: f64, t52481: f64, t52486: f64, t52488: f64, t52490: f64, t52492: f64, t52495: f64, t52499: f64, t3026: f64, t11616: f64, t4719: f64, t11598: f64, t11507: f64, t15266: f64, t11591: f64, t4725: f64, t15556: f64, t11105: f64, t11108: f64, t1699: f64, t3333: f64, t41937: f64, t5019: f64, t5023: f64, t52502: f64, t52507: f64, t52510: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52874, t52876, t52880, t52882, t52883) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2914(t300, t52368, t15547, t3030, t3012, t1634, t52239, t15520, t3022, t52481, t52486, t52488, t52490, t52492, t52495, t52499);
        let (t52885, t52887, t52889, t52897, t52899, t52905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2915(t15547, t3026, t11616, t4719, t11598, t11507, t300, t15266, t52239, t11591, t4725, t15556, t3022);
        let t52906 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2916(t11105, t11108, t1699, t3333, t41937, t5019, t5023, t52502, t52507, t52510, t52885, t52887, t52889, t52897, t52899, t52905);
    (t52874, t52876, t52880, t52882, t52883, t52885, t52887, t52889, t52897, t52899, t52905, t52906)
}
