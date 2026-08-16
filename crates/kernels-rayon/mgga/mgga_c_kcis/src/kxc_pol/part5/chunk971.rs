//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 971/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk971(t1160: f64, t318: f64, t86: f64, t284: f64, t3473: f64, t3177: f64, t3436: f64, t1194: f64, t381: f64, t1095: f64, t1169: f64, t983: f64) -> (f64, f64, f64, f64, f64) {
    let t10631 = t86 * t318 * t1160;
    let t10707 = t3473 * t284;
    let t10745 = t3177 * t3436;
    let t10752 = t381 * t1194;
    let t10753 = t1095 * t10752;
    let t10787 = t1169 * t983;
    (t10631, t10707, t10745, t10753, t10787)
}
