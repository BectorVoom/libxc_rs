//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 881/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk881<F: Float>(t19135: F, t3202: F, t3200: F, t167: F, t1767: F, t3203: F, t13172: F, t6626: F, t9425: F, t13122: F, t14570: F, t1710: F, t18511: F, t18515: F, t18517: F, t18521: F, t18523: F, t18528: F, t18532: F, t19115: F, t19118: F, t19121: F, t19124: F, t19128: F, t19130: F, t19132: F) -> (F, F, F, F, F) {
    let t19136 = t3202 * t19135;
    let t19137 = t3200 * t19136;
    let t19139 = t167 * t1767;
    let t19140 = t3203 * t19139;
    let t19141 = t3202 * t19140;
    let t19142 = t13172 * t19141;
    let t19144 = t9425 * t6626;
    let t19146 = -F::new(0.7369753086419753086e-3) * t13122 - F::new(0.33163888888888888888e-2) * t18511 + F::new(0.27636574074074074073e-2) * t18515 + F::new(0.33163888888888888888e-2) * t18517 - F::new(0.13345e0) * t14570 * t1710 - F::new(0.22109259259259259259e-2) * t18521 - F::new(0.66327777777777777776e-2) * t18523 + F::new(0.55273148148148148147e-2) * t18528 - F::new(0.36848765432098765431e-3) * t18532 - F::new(0.24872916666666666666e-2) * t19115 - F::new(0.33163888888888888888e-2) * t19118 - F::new(0.11054629629629629629e-2) * t19121 - F::new(0.88437037037037037035e-2) * t19124 + F::new(0.88437037037037037035e-2) * t19128 - F::new(0.22109259259259259259e-2) * t19130 - F::new(0.33163888888888888888e-2) * t19132 - F::new(0.33163888888888888888e-2) * t19137 - F::new(0.66327777777777777776e-2) * t19142 + F::new(0.22109259259259259258e-2) * t19144;
    (t19137, t19140, t19142, t19144, t19146)
}
