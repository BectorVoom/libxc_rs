//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 949/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk949(t12952: f64, t1375: f64, t11525: f64, t435: f64, t437: f64, t11529: f64, t447: f64, t445: f64, t3845: f64, t429: f64, t431: f64, t12868: f64, t1398: f64) -> (f64, f64, f64, f64, f64) {
    let t14047 = t1375 * t12952;
    let t14056 = 0.77488888888888888888e-2_f64 * t435 * t11525 * t437;
    let t14057 = t11529 * t447;
    let t14059 = 0.72818958333333333333e-4_f64 * t445 * t14057;
    let t14062 = 0.27323333333333333333e-1_f64 * t429 * t3845 * t431;
    let t14063 = t1398 * t12868;
    (t14047, t14056, t14059, t14062, t14063)
}
