//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1381;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta253<F: Float>(t819: F, t820: F, t9981: F, t2639: F, t2686: F, t2697: F, t2703: F, t842: F, t9612: F, t2617: F, t2696: F, t849: F, t847: F, t9516: F, t2645: F, t2647: F, t9621: F, t2618: F, t2623: F, t2630: F, t2635: F, t2643: F, t2681: F, t843: F, t9967: F, t9974: F, t9978: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9983, t9986, t9988, t9990, t9993) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1381::<F>(t819, t820, t9981, t2639, t2686, t2697, t2703, t842, t9612, t2617, t2696);
        let (t9994, t9997, t10003, t10006) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1382::<F>(t849, t9993, t820, t847, t9516, t2645, t2647, t9621, t2618, t2623, t2630, t2635, t2643, t2681, t2703, t843, t9967, t9974, t9978, t9983, t9986, t9988, t9990);
    (t9983, t9986, t9988, t9990, t9993, t9994, t9997, t10003, t10006)
}
