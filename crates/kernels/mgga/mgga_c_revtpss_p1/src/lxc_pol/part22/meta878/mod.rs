//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta878 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3045;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3046;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3047;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta878<F: Float>(t14575: F, t2435: F, t10943: F, t14598: F, t686: F, t72: F, t10541: F, t14495: F, t2782: F, t10518: F, t14568: F, t1568: F, t4503: F, t786: F, t10532: F, t40270: F, t4496: F, t136: F, t137: F, t14597: F, t2438: F, t2723: F, t49180: F, t836: F, t2457: F, t2710: F, t4469: F, t2722: F, t50474: F, t39597: F, t14586: F, t10529: F, t10115: F, t1576: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51537, t51541, t51544, t51546, t51548) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3045::<F>(t14575, t2435, t10943, t14598, t686, t72, t10541, t14495, t2782, t10518, t14568, t1568, t4503);
        let (t51550, t51553, t51560) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3046::<F>(t51548, t786, t10532, t40270, t4496, t136, t137, t14597, t2438, t2723, t49180, t836);
        let (t51564, t51572, t51576, t51578) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3047::<F>(t136, t2457, t2710, t4469, t2722, t50474, t2782, t39597, t14586, t10529, t10115, t1576);
    (t51537, t51541, t51544, t51546, t51548, t51550, t51553, t51560, t51564, t51572, t51576, t51578)
}
