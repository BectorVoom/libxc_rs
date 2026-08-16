//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 410/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk410(t2885: f64, t2888: f64, t119: f64, t56: f64, t69: f64, t45: f64, t5: f64, t157: f64, t849: f64, t52: f64, t840: f64, t846: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2890 = 0.16081824322151104822e2_f64 * t2885 * t2888;
    let t2892 = t69 * t119 * t56;
    let t2895 = t45 * t5;
    let t2896 = t157 * t849;
    let t2899 = t840 * t52;
    let t2900 = 1.0_f64 / t2899;
    let t2901 = t846 * t846;
    (t2890, t2892, t2895, t2896, t2900, t2901)
}
