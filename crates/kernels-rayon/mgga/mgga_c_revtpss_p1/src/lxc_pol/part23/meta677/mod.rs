//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2414;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta677(t13041: f64, t44173: f64, t13061: f64, t13100: f64, t828: f64, t12879: f64, t1247: f64, t1251: f64, t42994: f64, t1231: f64, t12898: f64, t43813: f64, t12256: f64, t3698: f64, t3362: f64, t414: f64, t12884: f64, t3555: f64, t3766: f64, t5330: f64, t1209: f64, t13147: f64, t17708: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44174, t44202, t44225, t44250, t44264, t44291, t44307) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2414(t13041, t44173, t13061, t13100, t828, t12879, t1247, t1251, t42994, t1231, t12898, t43813);
        let (t44348, t44361, t44425, t44484, t44500) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2415(t12256, t3698, t3362, t414, t12884, t828, t3555, t3766, t5330, t1209, t13147, t17708);
    (t44174, t44202, t44225, t44250, t44264, t44291, t44307, t44348, t44361, t44425, t44484, t44500)
}
