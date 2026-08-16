//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1296/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1296<F: Float>(t28938: F, t7703: F, t9938: F, t19294: F, t5329: F, t7691: F, t100540: F, t100547: F, t100553: F, t100558: F, t14497: F, t19399: F, t26695: F, t26739: F, t27936: F, t28948: F, t7690: F, t8042: F, t95963: F, t96000: F) -> (F, F) {
    let t101305 = t7703 * t9938 * t28938;
    let t101319 = t5329 * t7691 * t19294;
    let t101325 = F::cast_from(0.15445601851851851852e-3_f64) * t101305 + F::cast_from(0.61836467013888888889e-4_f64) * t95963 + F::cast_from(0.11054629629629629629e-1_f64) * t100540 - F::cast_from(0.12356481481481481482e-2_f64) * t7703 * t14497 * t26695 * t19399 + F::cast_from(0.22109259259259259259e-2_f64) * t100547 - F::cast_from(0.55273148148148148147e-3_f64) * t100553 + F::cast_from(0.49745833333333333332e-2_f64) * t100558 - F::cast_from(0.24734586805555555555e-3_f64) * t26739 * t28948 + F::cast_from(0.92754700520833333333e-4_f64) * t7690 * t101319 + F::cast_from(0.13901041666666666667e-2_f64) * t27936 * t8042 + F::cast_from(0.11054629629629629629e-2_f64) * t96000;
    (t101319, t101325)
}
