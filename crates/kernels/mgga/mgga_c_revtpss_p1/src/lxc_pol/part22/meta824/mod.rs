//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta824 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2941;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta824<F: Float>(t10026: F, t48084: F, t136: F, t2457: F, t3964: F, t5710: F, t221: F, t9817: F, t13792: F, t13845: F, t1882: F, t9994: F, t13793: F, t13999: F, t1868: F, t3923: F, t13872: F, t3978: F, t9921: F, t1320: F, t13632: F, t13672: F, t3860: F, t5567: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48085, t48089, t48100, t48102, t48105) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2941::<F>(t10026, t48084, t136, t2457, t3964, t5710, t221, t9817, t13792, t13845, t1882, t9994);
        let (t48111, t48113, t48143, t48152, t48154, t48158) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2942::<F>(t13793, t13999, t1868, t3923, t13872, t221, t3978, t9921, t1320, t13632, t13672, t3860, t5567);
    (t48085, t48089, t48100, t48102, t48105, t48111, t48113, t48143, t48152, t48154, t48158)
}
