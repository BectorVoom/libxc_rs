//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1119/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1119<F: Float>(t1388: F, t1410: F, t3944: F, t3950: F, t3956: F, t3967: F, t5606: F, t5625: F, t5666: F, t5681: F, t6846: F, t6850: F, t6856: F, t6887: F) -> F {
    let t6888 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t5681 + F::cast_from(0.20007875121765877254e-2_f64) * t5625 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t6846 + t3944 * t6850 / F::cast_from(16.0_f64) + t3950 + F::cast_from(0.80031500487063509015e-2_f64) * t5606 - F::cast_from(0.25410001404642664112e-4_f64) * t5666 - F::cast_from(0.85748036236139473944e-3_f64) * t1410 * t6856 + t3956 + t3967 + t6887;
    t6888
}
