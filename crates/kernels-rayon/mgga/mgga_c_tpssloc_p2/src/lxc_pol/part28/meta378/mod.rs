//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta378 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1445;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta378(t1147: f64, t1156: f64, t14829: f64, t1164: f64, t3423: f64, t4869: f64, t11126: f64, t1703: f64, t1657: f64, t3263: f64, t3266: f64, t11292: f64, t1694: f64, t3404: f64, t1098: f64, t4737: f64, t1119: f64, t3308: f64, t4740: f64, t3312: f64, t3316: f64, t11282: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14833, t14835, t14837, t14840, t14841) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1445(t1147, t1156, t14829, t1164, t3423, t4869, t11126, t1703, t1657, t3263, t3266, t11292, t1694);
        let (t14844, t14847, t14849, t14852, t14853) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1446(t14841, t3404, t1164, t1098, t4737, t1119, t3308, t4740, t1657, t3312, t3316, t11282, t1694);
    (t14833, t14835, t14837, t14840, t14844, t14847, t14849, t14852, t14853)
}
