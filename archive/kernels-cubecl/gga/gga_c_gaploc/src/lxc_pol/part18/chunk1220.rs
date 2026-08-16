//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1220/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1220<F: Float>(t27837: F, t27840: F, t27844: F, t27848: F, t27853: F, t27856: F, t27858: F, t27860: F, t471: F, t10657: F, t64: F, t3427: F, t90: F) -> (F, F, F) {
    let t32300 = (F::cast_from(189.0_f64) / F::cast_from(512.0_f64) * t27837 - F::cast_from(2499.0_f64) / F::cast_from(16384.0_f64) * t27840 + F::cast_from(1239.0_f64) / F::cast_from(524288.0_f64) * t27844 - F::cast_from(441.0_f64) / F::cast_from(0.16777216e8_f64) * t27848 + F::cast_from(147.0_f64) / F::cast_from(0.16777216e8_f64) * t27853 - F::cast_from(413.0_f64) / F::cast_from(524288.0_f64) * t27856 + F::cast_from(833.0_f64) / F::cast_from(16384.0_f64) * t27858 - F::cast_from(63.0_f64) / F::cast_from(512.0_f64) * t27860) * t471;
    let t32302 = F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t10657 * t64;
    let t32304 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3427 * t90;
    (t32300, t32302, t32304)
}
