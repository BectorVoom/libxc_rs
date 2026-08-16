//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1123/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1123(t9640: f64, t9642: f64, t9126: f64, t9129: f64, t9135: f64, t9139: f64, t9143: f64, t9148: f64, t9154: f64, t9160: f64, t9166: f64, t9172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44466 = 0.4726e1_f64 * t9640;
    let t44467 = 0.39914139006212695214e-1_f64 * t9642;
    let t44468 = 0.5987120850931904282e-1_f64 * t9126;
    let t44470 = 0.11974241701863808564e0_f64 * t9129;
    let t44472 = 0.5454932330849068346e-1_f64 * t9135;
    let t44473 = 0.3405167991463827152e-4_f64 * t9139;
    let t44474 = 0.1702583995731913576e-4_f64 * t9143;
    let t44475 = 0.212822999466489197e-4_f64 * t9148;
    let t44476 = 0.5107751987195740728e-4_f64 * t9154;
    let t44477 = 0.5107751987195740728e-4_f64 * t9160;
    let t44478 = 0.1702583995731913576e-4_f64 * t9166;
    let t44479 = 0.1702583995731913576e-4_f64 * t9172;
    (t44466, t44467, t44468, t44470, t44472, t44473, t44474, t44475, t44476, t44477, t44478, t44479)
}
