//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta772 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2742;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2743;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta772<F: Float>(t14869: F, t9775: F, t10899: F, t136: F, t216: F, t14786: F, t231: F, t40834: F, t854: F, t14833: F, t236: F, t2453: F, t9794: F, t125: F, t14662: F, t10777: F, t14671: F, t14917: F, t40725: F, t10811: F, t14678: F, t10871: F, t1558: F, t10627: F, t10639: F, t10666: F, t10786: F, t14767: F, t14785: F, t14791: F, t14894: F, t1544: F, t221: F, t2722: F, t2745: F, t2747: F, t40438: F, t40440: F, t40462: F, t4362: F, t4364: F, t4365: F, t4366: F, t4450: F, t50409: F, t50415: F, t50418: F, t50423: F, t50436: F, t775: F, t828: F, t837: F, t851: F, t10726: F, t10943: F, t2661: F, t4352: F, t14547: F, t40693: F, t2475: F, t2662: F, t14724: F) -> (F, F, F, F, F, F, F, F) {
        let (t50443, t50446, t50451, t50454, t50457) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2742::<F>(t14869, t9775, t10899, t136, t216, t14786, t231, t40834, t854, t14833, t236, t2453, t9794);
        let (t50459, t50474, t50480) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2743::<F>(t125, t14662, t10777, t14671, t14917, t40725, t10811, t14678, t10871, t1558, t10627, t10639, t10666, t10786, t14767, t14785, t14791, t14894, t1544, t221, t2722, t2745, t2747, t40438, t40440, t40462, t4362, t4364, t4365, t4366, t4450, t50409, t50415, t50418, t50423, t50436, t50443, t50446, t50454, t50457, t775, t828, t837, t851);
        let (t50493, t50497, t50502, t50504) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2744::<F>(t10726, t10943, t2661, t4352, t14547, t40693, t14917, t1558, t2475, t2662, t14724, t9775);
    (t50451, t50459, t50474, t50480, t50493, t50497, t50502, t50504)
}
