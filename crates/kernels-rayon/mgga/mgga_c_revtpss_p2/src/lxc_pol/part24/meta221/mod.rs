//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta221 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta221(t12051: f64, t357: f64, t11239: f64, t3143: f64, t342: f64, t3154: f64, t4980: f64, t994: f64, t4995: f64, t3057: f64, t3286: f64, t11627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12052, t12077, t12078, t12079, t12122, t12127, t12149, t12166) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk972(t12051, t357, t11239, t3143, t342, t3154, t4980, t994, t4995, t3057, t3286, t11627);
    (t12052, t12077, t12078, t12079, t12122, t12127, t12149, t12166)
}
