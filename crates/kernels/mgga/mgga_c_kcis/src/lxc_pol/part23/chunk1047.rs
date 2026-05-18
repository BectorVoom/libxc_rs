//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1047/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1047<F: Float>(t2642: F, t27438: F, t5701: F, t12246: F, t1598: F, t7895: F, t7904: F, t2239: F, t27359: F, t27369: F, t27372: F, t27416: F, t27420: F, t27425: F, t27429: F, t27432: F, t27435: F, t7898: F, t7908: F) -> (F, F, F, F, F) {
    let t27439 = t27438 * t2642;
    let t27440 = t5701 * t27439;
    let t27447 = t12246 * t1598;
    let t27450 = t7895 * t7904;
    let t27452 = F::new(0.92754700520833333333e-4) * t7898 * t27416 - F::new(0.55273148148148148147e-3) * t27420 - F::new(0.33163888888888888888e-2) * t27425 - F::new(0.33163888888888888888e-2) * t27429 + F::new(0.22109259259259259258e-2) * t27432 - F::new(0.23168402777777777778e-3) * t7908 * t27435 - F::new(0.30891203703703703704e-3) * t7908 * t27440 - F::new(0.13901041666666666667e-2) * t7908 * t27372 + F::new(0.61836467013888888889e-4) * t27369 * t27359 - F::new(0.69505208333333333333e-3) * t27447 * t2239 - F::new(0.46336805555555555556e-3) * t27450;
    (t27439, t27440, t27447, t27450, t27452)
}
