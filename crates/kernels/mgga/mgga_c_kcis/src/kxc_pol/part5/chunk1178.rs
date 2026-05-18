//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1178/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1178<F: Float>(t1131: F, t19763: F, t1096: F, t1092: F, t2855: F, t6330: F, t1021: F, t1020: F, t10450: F, t13399: F, t13409: F, t14065: F, t14079: F, t14081: F, t14086: F, t14103: F, t14104: F, t14390: F, t19107: F, t19738: F, t19743: F, t19747: F, t19752: F, t19754: F, t19759: F, t300: F) -> (F, F, F) {
    let t19764 = t1131 * t19763;
    let t19765 = t1096 * t19764;
    let t19766 = t1092 * t19765;
    let t19769 = t2855 * t6330;
    let t19770 = t1021 * t19769;
    let t19771 = t1020 * t19770;
    let t19775 = F::new(0.44218518518518518516e-2) * t13399 + t13409 - F::new(0.16581944444444444444e-2) * t19738 + F::new(0.11054629629629629629e-2) * t19743 + F::new(0.18424382716049382715e-2) * t19747 - F::new(0.11054629629629629629e-2) * t14065 + F::new(0.66327777777777777776e-2) * t19752 + F::new(0.16581944444444444444e-2) * t19754 - t14079 - F::new(0.55273148148148148147e-3) * t19759 - F::new(0.44218518518518518516e-2) * t14081 - t14086 + t14103 - F::new(0.22109259259259259259e-2) * t14104 - F::new(0.16581944444444444444e-2) * t19766 + t19107 * t300 + F::new(0.88437037037037037035e-2) * t19771 - F::new(0.55273148148148148147e-3) * t10450 - F::new(0.7369753086419753086e-3) * t14390;
    (t19766, t19771, t19775)
}
