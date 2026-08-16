//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta456 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1987;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1988;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1989;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta456(t14507: f64, t231: f64, t2783: f64, t2782: f64, t10073: f64, t4496: f64, t10542: f64, t4500: f64, t4424: f64, t72: f64, t686: f64, t2798: f64, t136: f64, t1559: f64, t2457: f64, t10535: f64, t10069: f64, t1568: f64, t836: f64, t10519: f64, t10524: f64, t10943: f64, t14498: f64, t14502: f64, t14506: f64, t4366: f64, t4494: f64, t4504: f64, t4514: f64, t837: f64, t10867: f64, t225: f64, t213: f64, t10871: f64, t2722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14509, t14511, t14512, t14518, t14519, t14520, t14522) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1987(t14507, t231, t2783, t2782, t10073, t4496, t10542, t4500, t4424, t72, t686, t2798);
        let (t14523, t14524, t14535, t14537, t14540) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1988(t136, t1559, t2457, t10535, t10069, t4496, t1568, t836, t231, t2783, t2782, t10519, t10524, t10943, t14498, t14502, t14506, t14507, t14511, t14512, t14518, t14522, t4366, t4494, t4504, t4514, t837);
        let (t14545, t14546) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1989(t10867, t225, t213);
        let t14547 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1990(t10871, t2722);
    (t14509, t14519, t14520, t14523, t14524, t14535, t14537, t14540, t14545, t14546, t14547)
}
