//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 819/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk819(t1173: f64, t7894: f64, t1171: f64, t7748: f64, t3722: f64, t7819: f64, t7779: f64, t827: f64, t7782: f64, t7776: f64, t45: f64, t7796: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25623 = t1173 * t7894;
    let t25663 = t7748 * t1171;
    let t25668 = t3722 * t7819;
    let t25696 = t827 * t7779;
    let t25699 = t827 * t7782;
    let t25701 = t827 * t7776;
    let t25786 = t45 * t7796;
    (t25623, t25663, t25668, t25696, t25699, t25701, t25786)
}
