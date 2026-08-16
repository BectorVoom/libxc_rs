//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk841;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk842;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta187<F: Float>(t2697: F, t2703: F, t842: F, t9612: F, t2617: F, t2696: F, t849: F, t820: F, t847: F, t9516: F, t2645: F, t2647: F, t9621: F, t2618: F, t2623: F, t2630: F, t2635: F, t2643: F, t2681: F, t843: F, t9967: F, t9974: F, t9978: F, t9983: F, t9986: F, t232: F, t2553: F, t2646: F, t2614: F, t838: F, t2693: F, t809: F, t225: F, t9584: F, t237: F, t597: F, t61: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9988, t9990, t9993, t9994, t9997, t10003) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk841::<F>(t2697, t2703, t842, t9612, t2617, t2696, t849, t820, t847, t9516, t2645, t2647, t9621);
        let t10006 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk842::<F>(t10003, t2618, t2623, t2630, t2635, t2643, t2681, t2703, t843, t849, t9967, t9974, t9978, t9983, t9986, t9988, t9990, t9994, t9997);
        let (t10007, t10009, t10012, t10014, t10016, t10017, t10021) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk843::<F>(t232, t2553, t2645, t2646, t2614, t838, t2693, t809, t225, t9584, t237, t597, t61);
    (t9990, t9993, t9997, t10003, t10006, t10007, t10009, t10012, t10014, t10016, t10017, t10021)
}
