//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta623 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2030;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2031;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta623(t11820: f64, t7339: f64, t2122: f64, t7319: f64, t1235: f64, t225: f64, t461: f64, t11553: f64, t2121: f64, t2123: f64, t7288: f64, t85660: f64, t3427: f64, t7295: f64, t11947: f64, t7394: f64, t2157: f64, t43706: f64, t1453: f64, t81439: f64, t26129: f64, t81442: f64, t22470: f64, t4067: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86350, t86403, t86415, t86451, t86473) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2030(t11820, t7339, t2122, t7319, t1235, t225, t461, t11553, t2121, t2123, t7288, t85660);
        let (t86501, t86517, t86524, t86586, t86589, t86590) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2031(t2121, t3427, t7295, t11947, t7394, t2157, t43706, t1453, t81439, t26129, t81442, t22470, t4067);
    (t86350, t86403, t86415, t86451, t86473, t86501, t86517, t86524, t86586, t86589, t86590)
}
