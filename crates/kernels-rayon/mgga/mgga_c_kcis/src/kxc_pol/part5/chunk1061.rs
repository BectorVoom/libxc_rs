//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1061/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1061(t11913: f64, t5638: f64, t1924: f64, t3960: f64, t1928: f64, t4169: f64, t1392: f64, t1981: f64, t1017: f64, t86: f64) -> (f64, f64, f64, f64, f64) {
    let t17276 = t11913 * t5638;
    let t17277 = 0.14739506172839506172e-2_f64 * t17276;
    let t17287 = t1924 * t3960;
    let t17292 = t4169 * t1928;
    let t17296 = t1392 * t1981;
    let t17298 = t86 * t1017 * t17296;
    (t17276, t17277, t17287, t17292, t17298)
}
