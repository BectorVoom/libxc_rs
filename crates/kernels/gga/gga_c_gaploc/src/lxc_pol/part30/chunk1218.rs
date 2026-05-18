//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1218/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1218<F: Float>(t27837: F, t27840: F, t27844: F, t27848: F, t27853: F, t27856: F, t27858: F, t27860: F, t471: F, t10657: F, t64: F, t3427: F, t90: F) -> (F, F, F) {
    let t32300 = (F::new(189.0) / F::new(512.0) * t27837 - F::new(2499.0) / F::new(16384.0) * t27840 + F::new(1239.0) / F::new(524288.0) * t27844 - F::new(441.0) / F::new(0.16777216e8) * t27848 + F::new(147.0) / F::new(0.16777216e8) * t27853 - F::new(413.0) / F::new(524288.0) * t27856 + F::new(833.0) / F::new(16384.0) * t27858 - F::new(63.0) / F::new(512.0) * t27860) * t471;
    let t32302 = F::new(8.0) / F::new(3.0) * t10657 * t64;
    let t32304 = F::new(4.0) / F::new(3.0) * t3427 * t90;
    (t32300, t32302, t32304)
}
