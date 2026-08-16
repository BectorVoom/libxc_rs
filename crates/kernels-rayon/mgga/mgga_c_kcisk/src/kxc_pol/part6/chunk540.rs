//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 540/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk540(t5061: f64, t5320: f64, t747: f64, t79: f64, t1965: f64, t2597: f64, t1676: f64, t2386: f64, t2394: f64, t4790: f64, t240: f64, t260: f64, t604: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7429 = t5061 * t5320;
    let t7430 = t79 * t747;
    let t7467 = t2597 * t1965;
    let t7498 = t2386 * t1676;
    let t7509 = t2394 * t4790;
    let t7517 = t240 * t2386;
    let t7567 = t260 * t67 * t604;
    (t7429, t7430, t7467, t7498, t7509, t7517, t7567)
}
