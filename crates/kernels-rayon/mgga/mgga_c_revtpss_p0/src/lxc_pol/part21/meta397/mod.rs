//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1851;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta397(t1256: f64, t3651: f64, t2434: f64, t371: f64, t482: f64, t481: f64, t3172: f64, t3605: f64, t3600: f64, t11262: f64, t1251: f64, t1247: f64, t3704: f64, t3708: f64, t1284: f64, t3566: f64, t3624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12895, t12898, t12900, t12901, t12902, t12904, t12905) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1851(t1256, t3651, t2434, t371, t482, t481, t3172, t3605, t3600, t11262, t1251, t1247);
        let (t12907, t12909, t12910) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1852(t3704, t3708, t1284, t3566, t3624);
    (t12895, t12898, t12900, t12901, t12902, t12904, t12905, t12907, t12909, t12910)
}
