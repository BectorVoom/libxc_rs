//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta137 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk766;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk767;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta137<F: Float>(t3362: F, t3698: F, t2251: F, t1012: F, t1251: F, t3172: F, t1247: F, t1032: F, t1204: F, t1246: F, t1234: F, t1260: F, t1214: F, t1263: F, t1122: F, t1042: F, t1209: F, t1284: F, t3624: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3699, t3700, t3701, t3704, t3705, t3708) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk766::<F>(t3362, t3698, t2251, t1012, t1251, t3172, t1247, t1032, t1204, t1246);
        let t3711 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk767::<F>(t1234, t1260);
        let (t3712, t3713, t3714, t3717, t3718) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk768::<F>(t1214, t1263, t1122, t1042, t1209, t1284, t3624);
    (t3699, t3700, t3701, t3704, t3705, t3708, t3711, t3712, t3713, t3714, t3717, t3718)
}
