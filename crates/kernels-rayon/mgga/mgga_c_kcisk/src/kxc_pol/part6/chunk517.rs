//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 517/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk517(t339: f64, t63: f64, t67: f64, t378: f64, t4143: f64, t1305: f64, t2160: f64, t1308: f64, t2159: f64) -> (f64, f64, f64, f64) {
    let t6141 = t339 * t63 * t67;
    let t6142 = t378 * t4143;
    let t6155 = t2160 * t1305;
    let t6157 = t2159 * t1308;
    (t6141, t6142, t6155, t6157)
}
