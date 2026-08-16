//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1207/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1207(t19890: f64, t5181: f64, t3437: f64, t19112: f64, t388: f64, t387: f64, t1187: f64, t1184: f64, t6732: f64, t19735: f64, t3338: f64, t3337: f64) -> (f64, f64, f64, f64) {
    let t20169 = t5181 * t19890;
    let t20170 = t3437 * t20169;
    let t20172 = t388 * t19112;
    let t20173 = t387 * t20172;
    let t20174 = t1187 * t20173;
    let t20176 = t1184 * t6732;
    let t20178 = t3338 * t19735;
    let t20179 = t3337 * t20178;
    (t20170, t20174, t20176, t20179)
}
