//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta738 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta738(t2435: f64, t9635: f64, t9590: f64, t9593: f64, t10179: f64, t1450: f64, t4146: f64, t1455: f64, t5808: f64, t46279: f64, t46281: f64, t46286: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47620, t47638, t47651, t47672, t47730, t47753, t47754, t47758) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2591(t2435, t9635, t9590, t9593, t10179, t1450, t4146, t1455, t5808, t46279, t46281, t46286);
    (t47620, t47638, t47651, t47672, t47730, t47753, t47754, t47758)
}
