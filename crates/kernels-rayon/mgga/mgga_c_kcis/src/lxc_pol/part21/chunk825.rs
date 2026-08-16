//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 825/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk825(t1098: f64, t3309: f64, t3255: f64, t3281: f64, t245: f64, t2840: f64, t347: f64, t313: f64, t3262: f64, t1035: f64, t1103: f64, t1018: f64, t932: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10284 = t1098 * t3309;
    let t10286 = t3255 * t3281;
    let t10292 = t2840 * t245 * t347;
    let t10297 = t3262 * t313;
    let t10314 = t1103 * t1035;
    let t10324 = t1018 * t932 * t347;
    (t10284, t10286, t10292, t10297, t10314, t10324)
}
