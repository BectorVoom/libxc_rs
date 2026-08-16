//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1647;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta345<F: Float>(t14473: F, t2439: F, t212: F, t4469: F, t780: F, t689: F, t1579: F, t2769: F, t886: F, t252: F, t2782: F, t2470: F, t4480: F, t2465: F, t1558: F, t836: F, t231: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14474, t14476, t14477, t14479, t14480, t14481, t14482, t14484, t14485) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1647::<F>(t14473, t2439, t212, t4469, t780, t689, t1579, t2769, t886, t252, t2782, t2470, t4480);
        let (t14486, t14494, t14495) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1648::<F>(t14485, t2465, t1558, t836, t231);
    (t14474, t14476, t14477, t14479, t14480, t14481, t14482, t14484, t14485, t14486, t14494, t14495)
}
