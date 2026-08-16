//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 711/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk711(t10933: f64, t3118: f64, t353: f64, t579: f64, t609: f64, t615: f64, t1709: f64, t4865: f64, t10937: f64, t10941: f64, t10944: f64, t10947: f64, t10951: f64, t10954: f64, t10957: f64, t10960: f64, t10963: f64, t10966: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11030 = 0.93011851851851851854e0_f64 * t10933;
    let t11032 = t353 * t3118 * t579;
    let t11033 = 0.73028148148148148147e0_f64 * t11032;
    let t11036 = 1.0_f64 / t609 / t615 / 8.0_f64;
    let t11037 = t4865 * t1709;
    let t11038 = t11036 * t11037;
    let t11040 = 28.0_f64 / 27.0_f64 * t10933;
    let t11051 = -t11040 - 4.0_f64 / 9.0_f64 * t10937 + 2.0_f64 / 9.0_f64 * t10941 - 2.0_f64 / 3.0_f64 * t10944 + t10947 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t10951 + 4.0_f64 / 3.0_f64 * t10954 - 2.0_f64 / 3.0_f64 * t10957 - 2.0_f64 * t10960 + 2.0_f64 * t10963 - t10966 / 3.0_f64;
    (t11030, t11032, t11033, t11037, t11038, t11051)
}
