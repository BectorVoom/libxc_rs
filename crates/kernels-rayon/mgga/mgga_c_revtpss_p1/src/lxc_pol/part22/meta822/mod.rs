//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta822 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2937;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta822(t14202: f64, t9303: f64, t14238: f64, t2453: f64, t10142: f64, t10019: f64, t14239: f64, t1882: f64, t4066: f64, t1398: f64, t21990: f64, t10022: f64, t2782: f64, t13790: f64, t4056: f64, t10073: f64, t14231: f64, t10139: f64, t14219: f64, t9285: f64, t14215: f64, t2470: f64, t4101: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48005, t48007, t48008, t48013, t48015, t48020, t48022) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2937(t14202, t9303, t14238, t2453, t10142, t10019, t14239, t1882, t4066, t1398, t21990, t10022, t2782);
        let (t48027, t48029, t48036, t48039) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2938(t13790, t4056, t10022, t2782, t10073, t14231, t10139, t14219, t9285, t14215, t2470, t4101);
    (t48005, t48007, t48008, t48013, t48015, t48020, t48022, t48027, t48029, t48036, t48039)
}
