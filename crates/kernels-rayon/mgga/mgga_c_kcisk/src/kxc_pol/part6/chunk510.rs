//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 510/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk510(t2059: f64, t3532: f64, t1390: f64, t5: f64, t969: f64, t1173: f64, t2188: f64, t2083: f64, t3598: f64, t1171: f64, t2079: f64, t3651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5670 = t3532 * t2059;
    let t5675 = t1390 * t2059;
    let t5680 = t5 * t969;
    let t5687 = t1173 * t2188;
    let t5690 = t3598 * t2083;
    let t5715 = t2079 * t1171;
    let t5730 = t3651 * t2083;
    (t5670, t5675, t5680, t5687, t5690, t5715, t5730)
}
