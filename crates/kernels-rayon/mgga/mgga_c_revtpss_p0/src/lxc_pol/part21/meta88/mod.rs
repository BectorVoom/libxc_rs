//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta88 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk623;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk624;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta88(t118: f64, t1502: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t508: f64, t511: f64, t569: f64, t651: f64, t3: f64, t117: f64, t1518: f64, param_d: f64, t572: f64, t573: f64, t76: f64, t84: f64, t198: f64, t207: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t1913, t1914, t1916, t1918) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk623(t118, t1502, t1519, t1843, t1847, t1911, t508, t511, t569, t651, t3, t117, t1518, param_d);
        let (t1921, t1927, t1940) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk624(t1916, t1918, t572, t573, t76, t84, t198, t207);
    (t1913, t1914, t1916, t1918, t1921, t1927, t1940)
}
