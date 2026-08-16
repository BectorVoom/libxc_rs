//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 706/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk706(t10252: f64, t1550: f64, t9732: f64, t9737: f64, t1756: f64, t2211: f64, t1356: f64, t570: f64, t9530: f64, t9740: f64, t1707: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10253 = t1550 * t10252;
    let t10254 = 0.11974241701863808564e0_f64 * t10253;
    let t10255 = 0.85129199786595678799e-5_f64 * t9732;
    let t10256 = 0.1702583995731913576e-4_f64 * t9737;
    let t10257 = t2211 * t1756;
    let t10258 = t1356 * t10257;
    let t10259 = 0.39914139006212695214e-1_f64 * t10258;
    let t10260 = t9530 * t570;
    let t10261 = t1356 * t10260;
    let t10262 = 0.79828278012425390428e-1_f64 * t10261;
    let t10263 = 0.17961362552795712846e0_f64 * t9740;
    let t10267 = t699 * t1707;
    (t10254, t10255, t10256, t10257, t10259, t10260, t10262, t10263, t10267)
}
