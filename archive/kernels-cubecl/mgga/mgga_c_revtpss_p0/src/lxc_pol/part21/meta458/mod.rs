//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1993;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta458<F: Float>(t14586: F, t836: F, t10529: F, t2782: F, t4469: F, t72: F, t686: F, t874: F, t1558: F, t2811: F, t2482: F, t122: F, t2723: F) -> (F, F, F, F, F, F, F, F) {
        let (t14587, t14588, t14590, t14593, t14596, t14597, t14598, t14600) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1993::<F>(t14586, t836, t10529, t2782, t4469, t72, t686, t874, t1558, t2811, t2482, t122, t2723);
    (t14587, t14588, t14590, t14593, t14596, t14597, t14598, t14600)
}
