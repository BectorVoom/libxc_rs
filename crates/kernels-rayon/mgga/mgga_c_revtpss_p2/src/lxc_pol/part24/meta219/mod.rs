//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta219 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk967;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk968;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta219(t271: f64, t2857: f64, t11144: f64, t11150: f64, t3252: f64, t283: f64, t66: f64, t3298: f64, t994: f64, t4891: f64, t3316: f64, t11132: f64, t126: f64, t373: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11821, t11822, t11827, t11852, t11853, t11858, t11859, t11874, t11875, t11890) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk967(t271, t2857, t11144, t11150, t3252, t283, t66, t3298, t994, t4891, t3316, t11132);
        let (t11921, t11922) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk968(t126, t373, t828);
    (t11821, t11822, t11827, t11852, t11853, t11858, t11859, t11874, t11875, t11890, t11921, t11922)
}
