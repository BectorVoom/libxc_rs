//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta861 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3011;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta861(t14724: f64, t9775: f64, t1558: f64, t2722: f64, t10726: f64, t2661: f64, t2724: f64, t4416: f64, t4352: f64, t10722: f64, t4435: f64, t14751: f64, t2652: f64, t14769: f64, t10716: f64, t14757: f64, t14772: f64, t221: f64, t2674: f64, t40683: f64, t2645: f64, t10868: f64, t2482: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50504, t50511, t50518, t50522, t50524, t50526) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3011(t14724, t9775, t1558, t2722, t10726, t2661, t2724, t4416, t4352, t10722, t4435, t14751, t2652);
        let (t50529, t50531, t50540, t50560, t50570) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3012(t14769, t2652, t10716, t14757, t14772, t221, t2674, t40683, t1558, t2645, t10868, t2482, t814);
    (t50504, t50511, t50518, t50522, t50524, t50526, t50529, t50531, t50540, t50560, t50570)
}
