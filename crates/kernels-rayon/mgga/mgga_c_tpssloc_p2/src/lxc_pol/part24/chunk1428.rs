//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1428/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1428(t22573: f64, t6875: f64, t22575: f64, t2319: f64, t6514: f64, t11968: f64, t1869: f64, t1976: f64, t2312: f64, t2320: f64, t23829: f64, t3929: f64, t510: f64, t650: f64, t6862: f64, t6872: f64, t83692: f64, t83694: f64, t83698: f64, t83853: f64, t83862: f64, t83866: f64, t83869: f64, t83876: f64, t83880: f64, t83882: f64, t83884: f64, t9347: f64, t9351: f64) -> (f64, f64) {
    let t83886 = t6875 * t22573;
    let t83888 = 18.0_f64 * t83886 * t22575;
    let t83889 = t6514 * t2319;
    let t83894 = -t11968 * t1869 - t1976 * t9347 - 6.0_f64 * t1976 * t9351 - 3.0_f64 * t2312 * t6862 - 6.0_f64 * t2320 * t6862 - 3.0_f64 * t23829 * t650 + 3.0_f64 * t3929 * t6872 - t510 * t83853 - 6.0_f64 * t510 * t83889 - t83692 - t83694 - t83698 + t83862 + t83866 - t83869 + t83876 + t83880 + t83882 + t83884 - t83888;
    (t83889, t83894)
}
