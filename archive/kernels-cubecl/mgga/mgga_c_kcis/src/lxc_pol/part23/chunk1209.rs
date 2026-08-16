//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1209/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1209<F: Float>(t17388: F, t28624: F, t97702: F, t97704: F, t97707: F, t97709: F, t97711: F, t97713: F, t97715: F, t97717: F, t97719: F, t97721: F, t97723: F, t97725: F, t97728: F, t97730: F, t97732: F, t97734: F, t97736: F) -> (F, F) {
    let t97738 = t28624 * t17388;
    let t97740 = -F::cast_from(0.1875e0_f64) * t97702 + F::cast_from(0.4046875e-1_f64) * t97704 + F::cast_from(0.12140625e0_f64) * t97707 - F::cast_from(0.20833333333333333333e-1_f64) * t97709 + F::cast_from(0.625e-1_f64) * t97711 - F::cast_from(0.10791666666666666667e0_f64) * t97713 - F::cast_from(0.125e0_f64) * t97715 - F::cast_from(0.5625e0_f64) * t97717 - F::cast_from(0.125e0_f64) * t97719 + F::cast_from(0.125e0_f64) * t97721 + F::cast_from(0.55555555555555555557e-1_f64) * t97723 - F::cast_from(0.9375e-1_f64) * t97725 + F::cast_from(0.5e0_f64) * t97728 + F::cast_from(0.21583333333333333334e0_f64) * t97730 + F::cast_from(0.625e-1_f64) * t97732 - F::cast_from(0.1875e0_f64) * t97734 + F::cast_from(0.89930555555555555557e-2_f64) * t97736 - F::cast_from(0.4046875e-1_f64) * t97738;
    (t97738, t97740)
}
