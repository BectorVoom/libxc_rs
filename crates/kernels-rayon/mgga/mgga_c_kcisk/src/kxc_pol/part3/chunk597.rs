//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 597/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk597(t1801: f64, t5063: f64, t5062: f64, t1869: f64, t1757: f64, t1894: f64, t1899: f64, t1873: f64, t140: f64, t1797: f64, t3737: f64, t1803: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5064 = t1801 * t5063;
    let t5065 = t5062 * t5064;
    let t5066 = t1869 * t5065;
    let t5068 = t1894 * t1757;
    let t5069 = t1899 * t5068;
    let t5070 = t1873 * t5069;
    let t5071 = t1869 * t5070;
    let t5074 = t140 * t3737 * t1797;
    let t5075 = t5074 * t1803;
    (t5064, t5065, t5066, t5068, t5069, t5070, t5071, t5074, t5075)
}
