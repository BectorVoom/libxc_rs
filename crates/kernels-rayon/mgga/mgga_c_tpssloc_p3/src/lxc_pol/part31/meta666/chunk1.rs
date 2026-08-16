//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1956/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1956(t1395: f64, t5456: f64, t2105: f64, t6470: f64, t1851: f64, t7961: f64, t1404: f64, t1858: f64, t20149: f64, t20186: f64, t2099: f64, t27241: f64, t29396: f64, t5364: f64, t5381: f64, t6483: f64, t7223: f64, t7946: f64, t91830: f64, t91832: f64, t91834: f64, t91842: f64) -> (f64, f64) {
    let t100930 = t1395 * t5456;
    let t100966 = t6470 * t2105;
    let t100972 = t1851 * t7961;
    let t100976 = t1404 * t29396 + 2.0_f64 * t1858 * t27241 + t20149 * t2105 + t20186 * t2099 + 2.0_f64 * t5364 * t7961 + 2.0_f64 * t5381 * t7946 + t6483 * t7223 + t100966 + 2.0_f64 * t100972 + t91830 + t91832 + t91834 + t91842;
    (t100930, t100976)
}
