//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1118;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1119;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1120;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta328<F: Float>(t14567: F, t786: F, t2801: F, t233: F, t4469: F, t869: F, t689: F, t2435: F, t4519: F, t1558: F, t2723: F, t836: F, t10529: F, t2782: F, t72: F, t686: F, t874: F, t2811: F, t2482: F, t122: F, t676: F, t879: F, t1531: F, t37: F, t4392: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14568, t14570, t14577, t14581, t14586, t14587) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1118::<F>(t14567, t786, t2801, t233, t4469, t869, t689, t2435, t4519, t1558, t2723, t836);
        let (t14590, t14596, t14598, t14600) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1119::<F>(t10529, t14587, t2782, t4469, t72, t686, t874, t1558, t2811, t2482, t122, t2723);
        let (t14603, t14608, t14613, t14616) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1120::<F>(t14600, t676, t836, t14598, t1558, t879, t2482, t2801, t1531, t37, t4392, t72);
    (t14568, t14570, t14577, t14581, t14586, t14590, t14596, t14603, t14608, t14613, t14616)
}
