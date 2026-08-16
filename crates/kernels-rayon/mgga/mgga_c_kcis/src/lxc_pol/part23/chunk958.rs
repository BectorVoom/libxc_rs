//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 958/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk958(t16060: f64, t5968: f64, t1317: f64, t1507: f64, t17636: f64, t5463: f64, t1517: f64, t167: f64, t4225: f64, t2026: f64, t752: f64, t3393: f64, t5973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17649 = t5968 * t16060;
    let t17656 = t1507 * t1317;
    let t17669 = t5463 * t17636;
    let t17673 = t1517 * t4225 * t167;
    let t17676 = t752 * t2026;
    let t17685 = 0.35374814814814814814e-1_f64 * t3393 * t5973;
    (t17649, t17656, t17669, t17673, t17676, t17685)
}
