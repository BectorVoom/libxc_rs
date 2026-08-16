//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta647 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2069;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2070;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta647(t90864: f64, t26433: f64, t6883: f64, t22716: f64, t7741: f64, t22704: f64, t5336: f64, t80798: f64, t22724: f64, t26436: f64, t26423: f64, t81159: f64, t215: f64, t22839: f64, t562: f64, t80854: f64, t1338: f64, t26328: f64, t26462: f64, t6914: f64, t22705: f64, t26414: f64, t81228: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90865, t90867, t90868, t90899, t90900, t90912) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2069(t90864, t26433, t6883, t22716, t7741, t22704, t5336, t80798, t22724, t26436, t26423, t81159);
        let (t90913, t90914, t90915, t90952, t90957, t90961) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2070(t90912, t215, t22839, t562, t80854, t1338, t26328, t26462, t6914, t22705, t26414, t81228);
    (t90865, t90867, t90868, t90899, t90900, t90913, t90914, t90915, t90952, t90957, t90961)
}
