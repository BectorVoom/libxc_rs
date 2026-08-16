//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 824/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk824(t1131: f64, t6555: f64, t1021: f64, t1092: f64, t1768: f64, t5026: f64, t1774: f64, t4999: f64, t3262: f64, t3263: f64, t6272: f64, t1662: f64, t1670: f64, t3269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6556 = t1131 * t6555;
    let t6557 = t1021 * t6556;
    let t6558 = t1092 * t6557;
    let t6560 = t5026 * t1768;
    let t6561 = t1092 * t6560;
    let t6563 = t4999 * t1774;
    let t6564 = t1092 * t6563;
    let t6570 = t3262 * t3263 * t6272;
    let t6574 = t3269 * t1662 * t1670;
    (t6556, t6557, t6558, t6560, t6561, t6563, t6564, t6570, t6574)
}
