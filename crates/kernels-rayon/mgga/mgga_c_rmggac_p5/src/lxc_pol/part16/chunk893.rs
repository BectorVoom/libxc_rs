//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 893/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk893(t10024: f64, t34761: f64, t1502: f64, t16503: f64, t3369: f64, t559: f64, t34975: f64, t34976: f64, t571: f64, t9145: f64, t8537: f64, t8659: f64) -> (f64, f64, f64, f64, f64) {
    let t44801 = t34761 * t10024;
    let t44808 = t16503 * t3369 * t559 * t1502;
    let t44812 = t34975 * t34976 * t571 * t9145;
    let t44816 = t34975 * t3369 * t559 * t9145;
    let t44818 = t8659 * t8537;
    (t44801, t44808, t44812, t44816, t44818)
}
