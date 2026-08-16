//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 878/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk878(t158: f64, t165: f64, t173: f64, t23238: f64, t23249: f64, t23251: f64, t23253: f64, t28645: f64, t28648: f64, t28651: f64, t28654: f64, t28657: f64, t28660: f64, t28663: f64, t5089: f64, t5168: f64) -> f64 {
    let t28671 = -0.39624999999999999999e-2_f64 * t23238 - 0.21078e-1_f64 * t158 * t28645 + 0.4755e-2_f64 * t165 * t28648 + 0.30247875e-4_f64 * t173 * t28651 + 0.317e-2_f64 * t165 * t28654 + 0.403305e-4_f64 * t173 * t28657 + 0.7925e-3_f64 * t165 * t28660 + 0.46615850170166761884e-3_f64 * t5168 * t28663 - 0.71734315950379065738e-1_f64 * t5089 * t28663 + 0.14052e-1_f64 * t23249 - 0.4684e-2_f64 * t23251 - 0.28104e-1_f64 * t23253;
    t28671
}
