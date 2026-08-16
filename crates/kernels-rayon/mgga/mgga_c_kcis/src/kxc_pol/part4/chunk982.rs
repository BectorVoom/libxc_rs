//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 982/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk982(t313: f64, t3262: f64, t1035: f64, t1103: f64, t1018: f64, t347: f64, t932: f64, t3255: f64, t3271: f64, t3276: f64, t3250: f64, t41: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10297 = t3262 * t313;
    let t10314 = t1103 * t1035;
    let t10324 = t1018 * t932 * t347;
    let t10333 = t3255 * t3271;
    let t10335 = t3255 * t3276;
    let t10338 = t85 * t3250 * t41;
    (t10297, t10314, t10324, t10333, t10335, t10338)
}
