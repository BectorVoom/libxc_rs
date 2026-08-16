//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 874/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk874(t15908: f64, t2375: f64, t1787: f64, t2516: f64, t17: f64, t2663: f64, t5157: f64, t1788: f64, t2225: f64, t2221: f64, t2223: f64, t12248: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15909 = t15908 * t2375;
    let t15971 = t1787 * t2516;
    let t15972 = t17 * t15971;
    let t15979 = t5157 * t2663;
    let t15982 = t2225 * t1788;
    let t15984 = t2221 * t1788;
    let t15986 = t2223 * t1788;
    let t16046 = t68 * t12248;
    (t15909, t15971, t15972, t15979, t15982, t15984, t15986, t16046)
}
