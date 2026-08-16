//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1359;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta248(t2885: f64, t919: f64, t2884: f64, t307: f64, t302: f64, t10294: f64, t10544: f64, t922: f64, t2887: f64, t310: f64, t2791: f64, t888: f64, t2929: f64, t938: f64, t10523: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10765, t10770, t10771) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1359(t2885, t919, t2884, t307, t302);
        let (t10784, t10785, t10810, t10811, t10813, t10817, t10825, t10828) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1360(t10294, t10544, t2884, t922, t302, t2887, t310, t2791, t888, t2929, t938, t10523, t315);
    (t10765, t10770, t10771, t10784, t10785, t10810, t10811, t10813, t10817, t10825, t10828)
}
