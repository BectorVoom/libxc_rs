//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1296/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1296(t28938: f64, t7703: f64, t9938: f64, t19294: f64, t5329: f64, t7691: f64, t100540: f64, t100547: f64, t100553: f64, t100558: f64, t14497: f64, t19399: f64, t26695: f64, t26739: f64, t27936: f64, t28948: f64, t7690: f64, t8042: f64, t95963: f64, t96000: f64) -> (f64, f64) {
    let t101305 = t7703 * t9938 * t28938;
    let t101319 = t5329 * t7691 * t19294;
    let t101325 = 0.15445601851851851852e-3_f64 * t101305 + 0.61836467013888888889e-4_f64 * t95963 + 0.11054629629629629629e-1_f64 * t100540 - 0.12356481481481481482e-2_f64 * t7703 * t14497 * t26695 * t19399 + 0.22109259259259259259e-2_f64 * t100547 - 0.55273148148148148147e-3_f64 * t100553 + 0.49745833333333333332e-2_f64 * t100558 - 0.24734586805555555555e-3_f64 * t26739 * t28948 + 0.92754700520833333333e-4_f64 * t7690 * t101319 + 0.13901041666666666667e-2_f64 * t27936 * t8042 + 0.11054629629629629629e-2_f64 * t96000;
    (t101319, t101325)
}
