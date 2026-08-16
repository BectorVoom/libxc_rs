//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta131 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk689;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk690;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk691;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk692;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta131<F: Float>(t487: F, t5219: F, t1770: F, t1209: F, t1811: F, t1256: F, t1804: F, t1786: F, t1796: F, t3172: F, t1247: F, t1263: F, t3367: F, t1032: F, t1246: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t5220 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk689::<F>(t487, t5219);
        let t5225 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk690::<F>(t1770, t487);
        let (t5251, t5254, t5256, t5265) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk691::<F>(t1209, t1811, t1256, t1804, t1786, t1796, t3172);
        let (t5266, t5268) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk692::<F>(t1247, t5265, t1263, t3367);
        let (t5273, t5274) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk693::<F>(t1032, t1770, t1246);
    (t5220, t5225, t5251, t5254, t5256, t5265, t5266, t5268, t5273, t5274)
}
