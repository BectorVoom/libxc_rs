//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1168/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1168(t19638: f64, t9410: f64, t3200: f64, t2861: f64, t6560: f64, t5019: f64, t5026: f64, t1092: f64, t13181: f64, t1774: f64, t2825: f64, t6556: f64) -> (f64, f64, f64, f64, f64) {
    let t19639 = t9410 * t19638;
    let t19640 = t3200 * t19639;
    let t19642 = t2861 * t6560;
    let t19644 = t5026 * t5019;
    let t19645 = t1092 * t19644;
    let t19647 = t13181 * t1774;
    let t19648 = t1092 * t19647;
    let t19650 = t2825 * t6556;
    (t19640, t19642, t19645, t19648, t19650)
}
