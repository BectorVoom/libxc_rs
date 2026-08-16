//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1055/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1055(t13330: f64, t9517: f64, t3200: f64, t3178: f64, t4985: f64, t1092: f64, t2825: f64, t4814: f64, t3182: f64, t4984: f64, t1096: f64, t1662: f64, t9476: f64) -> (f64, f64, f64, f64, f64) {
    let t13331 = t9517 * t13330;
    let t13332 = t3200 * t13331;
    let t13336 = t3178 * t4985;
    let t13337 = t1092 * t13336;
    let t13339 = t2825 * t4814;
    let t13340 = t1092 * t13339;
    let t13342 = t3182 * t4984;
    let t13343 = t1096 * t13342;
    let t13344 = t1092 * t13343;
    let t13346 = t9476 * t1662;
    (t13332, t13337, t13340, t13344, t13346)
}
