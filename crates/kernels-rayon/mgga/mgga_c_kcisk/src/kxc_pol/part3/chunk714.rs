//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 714/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk714(t10933: f64, t11032: f64, t10944: f64, t10947: f64, t10951: f64, t10954: f64, t10960: f64, t10966: f64, t11038: f64, t11052: f64, t11054: f64, t11057: f64, t11060: f64, t11063: f64) -> f64 {
    let t11091 = 0.93932222222222222223e0_f64 * t10933;
    let t11092 = 0.73586666666666666667e0_f64 * t11032;
    let t11099 = -0.60385000000000000001e0_f64 * t10944 + 0.30192500000000000001e0_f64 * t10947 - 0.33547222222222222222e0_f64 * t10951 + 0.12077e1_f64 * t10954 - 0.181155e1_f64 * t10960 - 0.301925e0_f64 * t10966 - t11091 - t11092 + 0.19419375e1_f64 * t11038 + 0.16504875e0_f64 * t11052 + 0.258925e1_f64 * t11054 - 0.412621875e-1_f64 * t11057 + 0.247573125e0_f64 * t11060 - 0.3883875e1_f64 * t11063;
    t11099
}
