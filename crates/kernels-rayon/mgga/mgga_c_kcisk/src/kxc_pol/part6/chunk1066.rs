//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1066/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1066(t1312: f64, t31483: f64, t30557: f64, t30561: f64, t30564: f64, t30567: f64, t30641: f64, t30644: f64, t30660: f64, t30662: f64, t30664: f64, t30668: f64, t6568: f64, t7804: f64) -> (f64, f64) {
    let t31484 = t1312 * t31483;
    let t31492 = 0.35089340384731224426e1_f64 * t6568 * t7804 - t30557 + t30561 - t30564 + t30567 + t30641 + t30644 + t30660 + t30662 + t30664 - t30668;
    (t31484, t31492)
}
