//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1018/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1018(t23078: f64, t23080: f64, t6584: f64, t6604: f64, t6606: f64, t2679: f64, t815: f64, t6605: f64, t2684: f64, t1891: f64, t22822: f64, t133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23081 = t23078 * t23080;
    let t23083 = t6584 * t6604;
    let t23084 = t23083 * t6606;
    let t23086 = t815 * t2679;
    let t23087 = t6605 * t23086;
    let t23089 = t815 * t2684;
    let t23090 = t6605 * t23089;
    let t23093 = t22822 * t1891;
    let t23094 = t23093 * t133;
    (t23081, t23083, t23084, t23086, t23087, t23089, t23090, t23093, t23094)
}
