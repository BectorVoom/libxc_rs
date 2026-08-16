//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 649/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk649(t10399: f64, t6666: f64, t5192: f64, t5182: f64, t6675: f64, t6674: f64, t140: f64, t3737: f64, t5180: f64, t5196: f64, t1797: f64, t1862: f64) -> (f64, f64, f64, f64, f64) {
    let t10400 = t6666 * t10399;
    let t10401 = t5192 * t10400;
    let t10402 = t5182 * t10401;
    let t10404 = t6675 * t10399;
    let t10405 = t5192 * t10404;
    let t10406 = t6674 * t10405;
    let t10409 = t140 * t3737 * t5180;
    let t10410 = t10409 * t5196;
    let t10412 = t1797 * t1862;
    (t10402, t10406, t10409, t10410, t10412)
}
