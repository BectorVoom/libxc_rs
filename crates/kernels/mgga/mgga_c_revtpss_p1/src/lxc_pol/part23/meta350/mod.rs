//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta350 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1656;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1657;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta350<F: Float>(t14600: F, t676: F, t836: F, t14598: F, t1558: F, t879: F, t2482: F, t2801: F, t1531: F, t37: F, t4392: F, t72: F, t757: F, t73: F, t830: F, t1544: F, t2475: F, t4343: F, t853: F, t124: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14602, t14603, t14605, t14606, t14608, t14613, t14616) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1656::<F>(t14600, t676, t836, t14598, t1558, t879, t2482, t2801, t1531, t37, t4392, t72);
        let (t14618, t14643, t14648, t14652, t14671) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1657::<F>(t14616, t757, t73, t830, t1544, t2475, t4343, t853, t124, t1558);
    (t14602, t14603, t14605, t14606, t14608, t14613, t14616, t14618, t14643, t14648, t14652, t14671)
}
