//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1442;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1443;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1444;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta377(t136: f64, t14795: f64, t1113: f64, t14744: f64, t11265: f64, t1661: f64, t3271: f64, t11243: f64, t3270: f64, t4756: f64, t1102: f64, t3279: f64, t4748: f64, t3287: f64, t4764: f64, t4772: f64, t699: f64, t1107: f64, t14758: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t14728: f64, t11211: f64, t11213: f64, t11369: f64, t11372: f64, t14702: f64, t14705: f64, t14708: f64, t14711: f64, t14713: f64, t14759: f64, t14776: f64, t14779: f64, t14782: f64, t14784: f64, t14787: f64, t14790: f64, t14793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14796, t14799, t14802, t14805, t14809, t14811) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1442(t136, t14795, t1113, t14744, t11265, t1661, t3271, t11243, t3270, t4756, t1102, t3279, t4748);
        let (t14814, t14816, t14818, t14824, t14827) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1443(t3287, t4756, t1102, t3279, t4764, t4772, t699, t1107, t14758, t11137, t11139, t11141, t11143, t14728, t14809, t14811);
        let t14829 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1444(t11211, t11213, t11369, t11372, t14702, t14705, t14708, t14711, t14713, t14759, t14776, t14779, t14782, t14784, t14787, t14790, t14793, t14796, t14799, t14802, t14805, t14827);
    (t14796, t14799, t14802, t14805, t14809, t14811, t14814, t14816, t14818, t14824, t14829)
}
