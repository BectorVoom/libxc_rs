//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2055;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2056;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta613<F: Float>(t25331: F, t27216: F, t212: F, t27265: F, t689: F, t780: F, t1568: F, t7063: F, t25410: F, t25413: F, t27299: F, t93281: F, t93317: F, t2439: F, t7774: F, t93170: F, t25304: F, t27212: F, t25301: F, t93371: F, t27286: F, t25431: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98825, t98830, t98848, t98849, t98851, t98852, t98853) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2055::<F>(t25331, t27216, t212, t27265, t689, t780, t1568, t7063, t25410, t25413, t27299, t93281);
        let (t98856, t98858, t98868, t98875, t98877, t98879) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2056::<F>(t93317, t98852, t2439, t7774, t93170, t25304, t27212, t25301, t93371, t27286, t689, t25431);
    (t98825, t98830, t98848, t98849, t98851, t98853, t98856, t98858, t98868, t98875, t98877, t98879)
}
