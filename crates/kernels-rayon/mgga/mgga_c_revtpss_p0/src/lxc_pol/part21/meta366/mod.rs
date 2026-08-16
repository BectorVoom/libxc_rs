//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1738;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1739;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1740;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta366(t12051: f64, t357: f64, t12048: f64, t1043: f64, t1089: f64, t3259: f64, t12032: f64, t380: f64, t11620: f64, t378: f64, t359: f64, t999: f64, t11239: f64, t3143: f64, t342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12052, t12053, t12057, t12066, t12070, t12073, t12074) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1738(t12051, t357, t12048, t1043, t1089, t3259, t12032, t380, t11620, t378, t359, t999);
        let t12077 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1739(t11239, t3143);
        let t12078 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1740(t12077, t342);
    (t12052, t12053, t12057, t12066, t12070, t12073, t12074, t12077, t12078)
}
