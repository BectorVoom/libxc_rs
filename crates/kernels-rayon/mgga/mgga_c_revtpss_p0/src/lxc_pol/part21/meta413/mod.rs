//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta413 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1885;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta413(t13225: f64, t3: f64, t2327: f64, t670: f64, t116: f64, t2371: f64, t10259: f64, t117: f64, t1459: f64, t1461: f64, t4158: f64, t4162: f64, t4165: f64, t572: f64, t573: f64, param_d: f64, t10270: f64, t10272: f64, t10279: f64, t10281: f64, t10288: f64, t10290: f64, t10275: f64, t10278: f64, t10284: f64, t10287: f64, t10295: f64, t4171: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13226, t13232, t13240, t13244, t13247, t13250) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1885(t13225, t3, t2327, t670, t116, t2371, t10259, t117, t1459, t1461, t4158, t4162, t4165, t572, t573, param_d);
        let (t13267, t13269) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1886(t10270, t10272, t10279, t10281, t10288, t10290, t10275, t10278, t10284, t10287, t10295, t4171, t602);
    (t13226, t13232, t13240, t13244, t13247, t13250, t13267, t13269)
}
