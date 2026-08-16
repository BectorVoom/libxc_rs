//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta496 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1496;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta496<F: Float>(t22352: F, t2435: F, t2289: F, t5916: F, t5892: F, t25048: F, t575: F, t22590: F, t625: F, t22593: F, t22629: F, t116: F, t22746: F) -> (F, F, F, F, F, F, F, F) {
        let (t75274, t75540, t75639, t75808, t75822, t75831, t75843, t75941) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1496::<F>(t22352, t2435, t2289, t5916, t5892, t25048, t575, t22590, t625, t22593, t22629, t116, t22746);
    (t75274, t75540, t75639, t75808, t75822, t75831, t75843, t75941)
}
