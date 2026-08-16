//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta689 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2509;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2510;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta689(t1222: f64, t3688: f64, t697: f64, t13001: f64, t140: f64, t1226: f64, t2438: f64, t12855: f64, t12857: f64, t12916: f64, t12956: f64, t12959: f64, t3566: f64, t3781: f64, t5330: f64, t3362: f64, t404: f64, t13007: f64, t13028: f64, t3700: f64, t43813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44925, t44928, t44931, t44938, t44949) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2509(t1222, t3688, t697, t13001, t140, t1226, t2438, t12855, t12857, t12916, t12956, t12959);
        let (t44952, t44958, t44965, t44972, t44980, t45000) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2510(t3566, t3781, t5330, t3362, t404, t1222, t13007, t140, t13028, t3700, t697, t43813);
    (t44925, t44928, t44931, t44938, t44949, t44952, t44958, t44965, t44972, t44980, t45000)
}
