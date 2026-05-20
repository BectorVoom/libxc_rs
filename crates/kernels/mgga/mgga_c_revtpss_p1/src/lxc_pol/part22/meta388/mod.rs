//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta388 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1955;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1956;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1957;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta388<F: Float>(t30: F, t13680: F, t512: F, t9856: F, t1468: F, t9605: F, t2: F, t3874: F, t1344: F, t13554: F, t22: F, t2257: F, t3834: F, t5574: F, t5577: F, t580: F, zeta_threshold: F, t33: F, t1711: F, t9617: F, t3881: F, t1348: F, t13569: F, t3351: F, t3842: F, t5582: F, t5585: F) -> (F, F, F, F, F) {
        let (t13682, t13683, t13687, t13700) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1955::<F>(t30, t13680, t512, t9856, t1468, t9605, t2, t3874, t1344, t13554, t22, t2257, t3834, t5574, t5577, t580, zeta_threshold);
        let (t13701, t13714) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1956::<F>(t33, t1711, t9617, t2, t3881, t1348, t13569, t22, t3351, t3842, t5582, t5585, t580, zeta_threshold);
        let t13716 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1957::<F>(t13700, t13714);
    (t13682, t13683, t13687, t13701, t13716)
}
