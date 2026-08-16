//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1265/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1265(t3701: f64, t6324: f64, t571: f64, t6347: f64, t12461: f64, t12087: f64, t12094: f64, t12103: f64, t12105: f64, t12109: f64, t12114: f64, t1307: f64, t1388: f64, t16497: f64, t1799: f64, t19678: f64, t19683: f64, t19684: f64, t19685: f64, t19686: f64, t19687: f64, t3918: f64, t5126: f64, t5160: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64) -> f64 {
    let t20077 = t6324 * t3701;
    let t20081 = t571 * t6347;
    let t20085 = t6324 * t12461;
    let t20092 = -3.0_f64 * t1307 * t20077 * t3918 + 6.0_f64 * t1307 * t20081 * t5126 + 2.0_f64 * t1388 * t20085 * t5160 + 6.0_f64 * t16497 * t1799 * t3918 + t12087 - t12094 + t12103 - t12105 - t12109 - t12114 - t19678 - t19683 + t19684 + t19685 - t19686 + t19687 + t9793 + t9797 - t9820 - t9824;
    t20092
}
