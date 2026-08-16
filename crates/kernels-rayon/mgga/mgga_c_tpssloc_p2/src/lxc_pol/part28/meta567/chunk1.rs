//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1845/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1845(t25038: f64, t4282: f64, t6646: f64, t9647: f64, t25251: f64, t87049: f64, t23012: f64, t7529: f64, t13380: f64, t22986: f64, t2647: f64, t13377: f64, t1880: f64, t1894: f64, t214: f64) -> (f64, f64, f64, f64, f64) {
    let t87076 = t25038 * t6646 * t4282 * t9647;
    let t87078 = t87049 * t25251;
    let t87080 = t23012 * t7529;
    let t87084 = t22986 * t6646 * t13380 * t2647;
    let t87092 = t1880 * t214 * t1894 * t13377;
    (t87076, t87078, t87080, t87084, t87092)
}
