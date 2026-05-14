//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1022/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1022<F: Float>(t31241: F, t35436: F, t35448: F, t35452: F, t35459: F, t35469: F, t37541: F, t37559: F, t37565: F, t39962: F, t39965: F, t39967: F, t39969: F, t39971: F, t39973: F, t39977: F, t39981: F) -> (F,) {
    let t39983 = t37541 - 0.41930789719472202756e-3 * t31241 - 0.80031500487063509015e-1 * t35436 + t35448 - t35452 - 0.25724410870841842183e-2 * t39962 + t37559 + t35459 - 0.51448821741683684367e-2 * t35469 + 0.25724410870841842183e-2 * t39965 + t37565 - 0.34299214494455789578e-2 * t39967 + 0.17149607247227894789e-2 * t39969 - 0.17149607247227894789e-2 * t39971 + 0.85748036236139473944e-3 * t39973 + 0.41930789719472202757e-3 * t39977 + 0.42874018118069736972e-3 * t39981;
    (t39983,)
}
