//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1381;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1382;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta253(t819: f64, t820: f64, t9981: f64, t2639: f64, t2686: f64, t2697: f64, t2703: f64, t842: f64, t9612: f64, t2617: f64, t2696: f64, t849: f64, t847: f64, t9516: f64, t2645: f64, t2647: f64, t9621: f64, t2618: f64, t2623: f64, t2630: f64, t2635: f64, t2643: f64, t2681: f64, t843: f64, t9967: f64, t9974: f64, t9978: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9983, t9986, t9988, t9990, t9993) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1381(t819, t820, t9981, t2639, t2686, t2697, t2703, t842, t9612, t2617, t2696);
        let (t9994, t9997, t10003, t10006) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1382(t849, t9993, t820, t847, t9516, t2645, t2647, t9621, t2618, t2623, t2630, t2635, t2643, t2681, t2703, t843, t9967, t9974, t9978, t9983, t9986, t9988, t9990);
    (t9983, t9986, t9988, t9990, t9993, t9994, t9997, t10003, t10006)
}
