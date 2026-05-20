//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta766 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2716;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2717;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2718;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta766<F: Float>(t10428: F, t4305: F, t2609: F, t4186: F, t706: F, t10436: F, t4311: F, t14426: F, t72: F, t757: F, t18875: F, t2403: F, t2411: F, t2832: F, t39786: F, t39791: F, t39795: F, t39799: F, t39807: F, t39813: F, t45: F, t39858: F, t14386: F, t2414: F, t39860: F, t10326: F, t10356: F, t10446: F, t11231: F, t13312: F, t14401: F, t14404: F, t1469: F, t2251: F, t2258: F, t2375: F, t39825: F, t4377: F, t49889: F, t606: F, t78: F, zeta_threshold: F, t57: F, t10457: F, t14413: F, t14416: F, t2382: F, t39840: F, t4384: F, t81: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t49979, t49982, t49984, t49987, t49988) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2716::<F>(t10428, t4305, t2609, t4186, t706, t10436, t4311, t14426, t72, t757, t18875, t2403, t2411, t2832, t39786, t39791, t39795, t39799, t39807, t39813);
        let (t49992, t49994, t49995, t50014) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2717::<F>(t45, t39858, t14386, t2414, t39860, t10326, t10356, t10446, t11231, t13312, t14401, t14404, t1469, t2251, t2258, t2375, t39825, t4186, t4377, t49889, t606, t78, zeta_threshold);
        let t50033 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2718::<F>(t57, t10326, t10356, t10457, t11231, t13312, t14413, t14416, t1469, t2251, t2258, t2382, t39840, t4186, t4384, t49889, t606, t81, zeta_threshold);
    (t49979, t49982, t49984, t49987, t49988, t49992, t49994, t49995, t50014, t50033)
}
