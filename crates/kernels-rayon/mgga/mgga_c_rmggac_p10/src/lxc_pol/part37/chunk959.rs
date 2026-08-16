//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 959/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk959(t74994: f64, t16503: f64, t16504: f64, t699: f64, t9151: f64, t3369: f64, t9157: f64, t74997: f64, t69060: f64, t2333: f64, t71404: f64, t14589: f64, t8568: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77353 = 0.1276937996798935182e-4_f64 * t74994;
    let t77356 = t16503 * t16504 * t699 * t9151;
    let t77357 = 0.12769379967989351819e-4_f64 * t77356;
    let t77360 = t16503 * t3369 * t699 * t9157;
    let t77361 = 0.12769379967989351819e-4_f64 * t77360;
    let t77362 = 0.14967802127329760705e-1_f64 * t74997;
    let t77363 = 0.16263363996404810741e-4_f64 * t69060;
    let t77364 = t71404 * t2333;
    let t77365 = 0.68186654135613354322e-2_f64 * t77364;
    let t77366 = t14589 * t8568;
    (t77353, t77357, t77361, t77362, t77363, t77365, t77366)
}
