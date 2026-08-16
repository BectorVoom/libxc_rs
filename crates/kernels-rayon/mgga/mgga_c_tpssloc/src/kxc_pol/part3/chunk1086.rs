//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1086/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1086(t1539: f64, t3132: f64, t3071: f64, t3041: f64, t1616: f64, t2776: f64, t13969: f64, t4584: f64, t1041: f64, t4589: f64, t12652: f64, t4583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14121 = t1539 * t3132;
    let t14122 = t3071 * t14121;
    let t14125 = t1539 * t3041;
    let t14126 = t3071 * t14125;
    let t14129 = t1616 * t2776;
    let t14130 = t3071 * t14129;
    let t14134 = t13969 * t4584;
    let t14136 = t1041 * t14134 / 1728.0_f64;
    let t14137 = t13969 * t4589;
    let t14139 = 5.0_f64 / 10368.0_f64 * t1041 * t14137;
    let t14142 = t4583 * t12652;
    (t14122, t14126, t14130, t14136, t14139, t14142)
}
