//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1147/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1147(t23097: f64, t2679: f64, t776: f64, t815: f64, t23061: f64, t6604: f64, t23099: f64, t6605: f64, t9661: f64, t232: f64, t47320: f64, t1891: f64, t1895: f64, t213: f64, t39041: f64) -> (f64, f64, f64, f64, f64) {
    let t81833 = t23097 * t815 * t2679 * t776;
    let t81835 = t23061 * t6604;
    let t81836 = t81835 * t23099;
    let t81839 = t6605 * t815 * t9661;
    let t81843 = t23097 * t815 * t47320 * t232;
    let t81849 = t39041 * t1891 * t213 * t1895;
    (t81833, t81836, t81839, t81843, t81849)
}
