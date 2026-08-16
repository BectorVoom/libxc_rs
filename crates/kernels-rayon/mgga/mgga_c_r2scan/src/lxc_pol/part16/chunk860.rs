//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 860/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk860(t166: f64, t8590: f64, t3034: f64, t607: f64, t2483: f64, t955: f64, t2461: f64, t898: f64, t6030: f64, t7126: f64, t765: f64, t7898: f64, t7904: f64, t8649: f64, t8651: f64, t8652: f64) -> (f64, f64, f64, f64, f64) {
    let t9063 = t8590 * t166;
    let t9066 = t3034 * t607;
    let t9069 = t2483 * t955;
    let t9072 = t898 * t2461;
    let t9075 = -0.1143056e0_f64 * t7898 + 0.1350520664e0_f64 * t6030 - t8649 + t8651 + 0.675260332e-1_f64 * t765 * t9063 + 0.675260332e-1_f64 * t765 * t9066 + 0.1350520664e0_f64 * t765 * t9069 + 0.1350520664e0_f64 * t765 * t9072 - t7126 - t8652 - t7904;
    (t9063, t9066, t9069, t9072, t9075)
}
