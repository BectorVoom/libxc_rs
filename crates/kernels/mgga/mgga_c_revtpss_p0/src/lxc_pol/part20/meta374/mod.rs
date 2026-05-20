//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta374 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1356;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1357;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta374<F: Float>(t2723: F, t40262: F, t10666: F, t221: F, t2484: F, t2485: F, t2482: F, t2719: F, t596: F, t10852: F, t2645: F, t10858: F, t10863: F, t10868: F, t820: F, t843: F, t10874: F, t27: F, t10872: F, t10832: F, t10845: F, t823: F, t9948: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t40326, t40333, t40337, t40339, t40340, t40345) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1356::<F>(t2723, t40262, t10666, t221, t2484, t2485, t2482, t2719, t596, t10852, t2645, t10858, t10863);
        let (t40349, t40355, t40357, t40360) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1357::<F>(t10868, t820, t843, t10874, t2482, t27, t10872, t221, t2485, t10832, t10845, t823, t9948);
    (t40326, t40333, t40337, t40339, t40340, t40345, t40349, t40355, t40357, t40360)
}
