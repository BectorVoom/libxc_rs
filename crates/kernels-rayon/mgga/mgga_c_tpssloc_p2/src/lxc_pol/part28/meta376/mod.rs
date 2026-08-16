//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1440;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta376(t1100: f64, t14758: f64, t1667: f64, t2403: f64, t14720: f64, t11215: f64, t11217: f64, t14722: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64, t11219: f64, t14726: f64, t136: f64, t4775: f64, t699: f64, t14736: f64, t3297: f64, t14740: f64, t14731: f64, t1113: f64, t14749: f64, t14753: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14759, t14766, t14776) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1440(t1100, t14758, t1667, t2403, t14720, t11215, t11217, t14722, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t14779, t14781, t14782, t14784, t14787, t14790, t14793, t14795) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1441(t11219, t14726, t136, t4775, t699, t14736, t3297, t14740, t14731, t1113, t14749, t14753);
    (t14759, t14766, t14776, t14779, t14781, t14782, t14784, t14787, t14790, t14793, t14795)
}
