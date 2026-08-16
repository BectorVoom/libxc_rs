//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta766 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2716;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2717;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2718;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta766(t10428: f64, t4305: f64, t2609: f64, t4186: f64, t706: f64, t10436: f64, t4311: f64, t14426: f64, t72: f64, t757: f64, t18875: f64, t2403: f64, t2411: f64, t2832: f64, t39786: f64, t39791: f64, t39795: f64, t39799: f64, t39807: f64, t39813: f64, t45: f64, t39858: f64, t14386: f64, t2414: f64, t39860: f64, t10326: f64, t10356: f64, t10446: f64, t11231: f64, t13312: f64, t14401: f64, t14404: f64, t1469: f64, t2251: f64, t2258: f64, t2375: f64, t39825: f64, t4377: f64, t49889: f64, t606: f64, t78: f64, zeta_threshold: f64, t57: f64, t10457: f64, t14413: f64, t14416: f64, t2382: f64, t39840: f64, t4384: f64, t81: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49979, t49982, t49984, t49987, t49988) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2716(t10428, t4305, t2609, t4186, t706, t10436, t4311, t14426, t72, t757, t18875, t2403, t2411, t2832, t39786, t39791, t39795, t39799, t39807, t39813);
        let (t49992, t49994, t49995, t50014) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2717(t45, t39858, t14386, t2414, t39860, t10326, t10356, t10446, t11231, t13312, t14401, t14404, t1469, t2251, t2258, t2375, t39825, t4186, t4377, t49889, t606, t78, zeta_threshold);
        let t50033 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2718(t57, t10326, t10356, t10457, t11231, t13312, t14413, t14416, t1469, t2251, t2258, t2382, t39840, t4186, t4384, t49889, t606, t81, zeta_threshold);
    (t49979, t49982, t49984, t49987, t49988, t49992, t49994, t49995, t50014, t50033)
}
