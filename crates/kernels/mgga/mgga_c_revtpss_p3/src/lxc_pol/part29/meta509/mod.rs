//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta509 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1827;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta509<F: Float>(t1927: F, t2315: F, t2247: F, t2259: F, t2411: F, t605: F, t268: F, t41040: F, t837: F, t1032: F, t2760: F, t867: F, t7063: F, t1955: F, t25308: F, t2769: F, t7036: F, t820: F, t844: F, t2751: F, t2482: F, t814: F, t10782: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92584, t92588, t92790, t92840, t92888, t92889) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1827::<F>(t1927, t2315, t2247, t2259, t2411, t605, t268, t41040, t837, t1032, t2760, t867);
        let (t92890, t92917, t92951, t92952, t92955, t92956) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1828::<F>(t7063, t92889, t1955, t25308, t2769, t7036, t820, t844, t2751, t2482, t814, t10782);
    (t92584, t92588, t92790, t92840, t92888, t92889, t92890, t92917, t92951, t92952, t92955, t92956)
}
