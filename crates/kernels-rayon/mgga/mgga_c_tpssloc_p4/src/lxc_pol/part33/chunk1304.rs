//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1304/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1304(t1858: f64, t7758: f64, t2029: f64, t6470: f64, t1851: f64, t7774: f64, t1390: f64, t20416: f64, t1983: f64, t6878: f64, t20085: f64, t7753: f64) -> (f64, f64, f64, f64, f64) {
    let t100949 = t7758 * t1858;
    let t100952 = t6470 * t2029;
    let t100960 = t1851 * t7774;
    let t105159 = t1390 * t20416;
    let t105162 = 3.0_f64 * t1983 * t6878 * t105159;
    let t105165 = 6.0_f64 * t1983 * t7753 * t20085;
    (t100949, t100952, t100960, t105162, t105165)
}
