//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1360;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta415(t3154: f64, t42871: f64, t1036: f64, t42860: f64, t42866: f64, t357: f64, t11628: f64, t11631: f64, t3144: f64, t2434: f64, t246: f64, t3057: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42872, t42873, t42920, t42921, t42977, t42978, t42984, t42985, t42994, t43043) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1360(t3154, t42871, t1036, t42860, t42866, t357, t11628, t11631, t3144, t2434, t246, t3057, t3316);
    (t42872, t42873, t42920, t42921, t42977, t42978, t42984, t42985, t42994, t43043)
}
