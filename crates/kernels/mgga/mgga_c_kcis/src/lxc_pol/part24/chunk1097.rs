//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1097/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1097<F: Float>(t14628: F, t1773: F, t26760: F, t1092: F, t27895: F, t27947: F, t28948: F, t28952: F, t28967: F, t28974: F, t28984: F, t28988: F, t7690: F, t7703: F, t8030: F, t8034: F, t8042: F) -> (F, F, F, F) {
    let t28991 = t14628 * t1773;
    let t28992 = t26760 * t28991;
    let t28993 = t1092 * t28992;
    let t28995 = F::new(0.33163888888888888888e-2) * t28967 - F::new(0.2782641015625e-3) * t7690 * t28952 + F::new(0.13901041666666666667e-2) * t8030 * t8042 - F::new(0.24872916666666666666e-2) * t28974 + F::new(0.13901041666666666667e-2) * t8030 * t8034 + F::new(0.18550940104166666667e-3) * t27895 * t8034 + F::new(0.92754700520833333333e-4) * t7690 * t28948 + F::new(0.33163888888888888888e-2) * t27947 + F::new(0.46336805555555555556e-3) * t7703 * t28984 - F::new(0.13901041666666666667e-2) * t7703 * t28988 - F::new(0.33163888888888888888e-2) * t28993;
    (t28991, t28992, t28993, t28995)
}
