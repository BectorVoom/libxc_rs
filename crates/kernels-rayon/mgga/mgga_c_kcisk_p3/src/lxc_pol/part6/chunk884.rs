//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 884/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk884(t1801: f64, t28389: f64, t1800: f64, t1799: f64, t28369: f64, t10461: f64, t22254: f64, t2473: f64, t6719: f64, t8954: f64, t6974: f64, t8677: f64) -> (f64, f64, f64, f64, f64) {
    let t28756 = t1801 * t28389;
    let t28757 = t1800 * t28756;
    let t28758 = t1799 * t28757;
    let t28760 = t1801 * t28369;
    let t28761 = t1800 * t28760;
    let t28762 = t10461 * t28761;
    let t28764 = t22254 * t2473;
    let t28765 = t1799 * t28764;
    let t28767 = t6719 * t8954;
    let t28768 = t1799 * t28767;
    let t28775 = t6974 * t8677;
    (t28758, t28762, t28765, t28768, t28775)
}
