//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1987;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1988;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1989;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta456<F: Float>(t14507: F, t231: F, t2783: F, t2782: F, t10073: F, t4496: F, t10542: F, t4500: F, t4424: F, t72: F, t686: F, t2798: F, t136: F, t1559: F, t2457: F, t10535: F, t10069: F, t1568: F, t836: F, t10519: F, t10524: F, t10943: F, t14498: F, t14502: F, t14506: F, t4366: F, t4494: F, t4504: F, t4514: F, t837: F, t10867: F, t225: F, t213: F, t10871: F, t2722: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t14509, t14511, t14512, t14518, t14519, t14520, t14522) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1987::<F>(t14507, t231, t2783, t2782, t10073, t4496, t10542, t4500, t4424, t72, t686, t2798);
        let (t14523, t14524, t14535, t14537, t14540) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1988::<F>(t136, t1559, t2457, t10535, t10069, t4496, t1568, t836, t231, t2783, t2782, t10519, t10524, t10943, t14498, t14502, t14506, t14507, t14511, t14512, t14518, t14522, t4366, t4494, t4504, t4514, t837);
        let (t14545, t14546) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1989::<F>(t10867, t225, t213);
        let t14547 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1990::<F>(t10871, t2722);
    (t14509, t14519, t14520, t14523, t14524, t14535, t14537, t14540, t14545, t14546, t14547)
}
