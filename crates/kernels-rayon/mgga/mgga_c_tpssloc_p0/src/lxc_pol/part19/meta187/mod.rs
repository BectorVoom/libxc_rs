//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk841;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk842;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta187(t2697: f64, t2703: f64, t842: f64, t9612: f64, t2617: f64, t2696: f64, t849: f64, t820: f64, t847: f64, t9516: f64, t2645: f64, t2647: f64, t9621: f64, t2618: f64, t2623: f64, t2630: f64, t2635: f64, t2643: f64, t2681: f64, t843: f64, t9967: f64, t9974: f64, t9978: f64, t9983: f64, t9986: f64, t232: f64, t2553: f64, t2646: f64, t2614: f64, t838: f64, t2693: f64, t809: f64, t225: f64, t9584: f64, t237: f64, t597: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9988, t9990, t9993, t9994, t9997, t10003) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk841(t2697, t2703, t842, t9612, t2617, t2696, t849, t820, t847, t9516, t2645, t2647, t9621);
        let t10006 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk842(t10003, t2618, t2623, t2630, t2635, t2643, t2681, t2703, t843, t849, t9967, t9974, t9978, t9983, t9986, t9988, t9990, t9994, t9997);
        let (t10007, t10009, t10012, t10014, t10016, t10017, t10021) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk843(t232, t2553, t2645, t2646, t2614, t838, t2693, t809, t225, t9584, t237, t597, t61);
    (t9990, t9993, t9997, t10003, t10006, t10007, t10009, t10012, t10014, t10016, t10017, t10021)
}
