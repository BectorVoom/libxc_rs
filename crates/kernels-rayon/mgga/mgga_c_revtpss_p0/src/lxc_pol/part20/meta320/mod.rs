//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1230;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta320(t2434: f64, t371: f64, t482: f64, t481: f64, t3172: f64, t3605: f64, t3600: f64, t11262: f64, t1251: f64, t1247: f64, t3704: f64, t3708: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12898, t12900, t12901, t12902, t12904, t12905, t12907) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1230(t2434, t371, t482, t481, t3172, t3605, t3600, t11262, t1251, t1247, t3704, t3708);
    (t12898, t12900, t12901, t12902, t12904, t12905, t12907)
}
