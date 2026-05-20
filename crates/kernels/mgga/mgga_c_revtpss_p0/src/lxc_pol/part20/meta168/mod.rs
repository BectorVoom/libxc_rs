//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk897;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk898;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta168<F: Float>(t1317: F, t3853: F, t3829: F, t4140: F, t5536: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9365: F, t9374: F, t9376: F, t9389: F, t9391: F, t9394: F, t1320: F, t4029: F, t1353: F, t3855: F, t1333: F, t3863: F, t27: F, t583: F, t521: F, t19: F, t596: F, t182: F, t2490: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9396, t9397) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk897::<F>(t1317, t3853, t3829, t4140, t5536, t9278, t9308, t9316, t9320, t9325, t9329, t9333, t9365, t9374, t9376, t9389, t9391, t9394);
        let (t9399, t9400) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk898::<F>(t1320, t4029, t1353, t3829);
        let (t9405, t9407, t9409, t9410, t9412, t9413, t9415, t9417) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk899::<F>(t1317, t3855, t4029, t1333, t3863, t27, t583, t521, t19, t596, t182, t2490);
    (t9396, t9397, t9399, t9400, t9405, t9407, t9409, t9410, t9412, t9413, t9415, t9417)
}
