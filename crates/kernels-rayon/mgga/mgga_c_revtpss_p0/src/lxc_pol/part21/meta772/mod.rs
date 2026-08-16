//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta772 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2742;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2743;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta772(t14869: f64, t9775: f64, t10899: f64, t136: f64, t216: f64, t14786: f64, t231: f64, t40834: f64, t854: f64, t14833: f64, t236: f64, t2453: f64, t9794: f64, t125: f64, t14662: f64, t10777: f64, t14671: f64, t14917: f64, t40725: f64, t10811: f64, t14678: f64, t10871: f64, t1558: f64, t10627: f64, t10639: f64, t10666: f64, t10786: f64, t14767: f64, t14785: f64, t14791: f64, t14894: f64, t1544: f64, t221: f64, t2722: f64, t2745: f64, t2747: f64, t40438: f64, t40440: f64, t40462: f64, t4362: f64, t4364: f64, t4365: f64, t4366: f64, t4450: f64, t50409: f64, t50415: f64, t50418: f64, t50423: f64, t50436: f64, t775: f64, t828: f64, t837: f64, t851: f64, t10726: f64, t10943: f64, t2661: f64, t4352: f64, t14547: f64, t40693: f64, t2475: f64, t2662: f64, t14724: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50443, t50446, t50451, t50454, t50457) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2742(t14869, t9775, t10899, t136, t216, t14786, t231, t40834, t854, t14833, t236, t2453, t9794);
        let (t50459, t50474, t50480) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2743(t125, t14662, t10777, t14671, t14917, t40725, t10811, t14678, t10871, t1558, t10627, t10639, t10666, t10786, t14767, t14785, t14791, t14894, t1544, t221, t2722, t2745, t2747, t40438, t40440, t40462, t4362, t4364, t4365, t4366, t4450, t50409, t50415, t50418, t50423, t50436, t50443, t50446, t50454, t50457, t775, t828, t837, t851);
        let (t50493, t50497, t50502, t50504) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2744(t10726, t10943, t2661, t4352, t14547, t40693, t14917, t1558, t2475, t2662, t14724, t9775);
    (t50451, t50459, t50474, t50480, t50493, t50497, t50502, t50504)
}
