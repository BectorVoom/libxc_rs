//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1222/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1222<F: Float>(t871: F, t8710: F, t27837: F, t27840: F, t27844: F, t27856: F, t27858: F, t27860: F, t32300: F, t32302: F, t32304: F, t739: F) -> (F, F) {
    let t32305 = t8710 * t871;
    let t32307 = F::cast_from(63.0_f64) / F::cast_from(512.0_f64) * t27837;
    let t32308 = F::cast_from(385.0_f64) / F::cast_from(16384.0_f64) * t27840;
    let t32309 = F::cast_from(147.0_f64) / F::cast_from(1048576.0_f64) * t27844;
    let t32310 = F::cast_from(49.0_f64) / F::cast_from(1048576.0_f64) * t27856;
    let t32311 = F::cast_from(385.0_f64) / F::cast_from(49152.0_f64) * t27858;
    let t32312 = F::cast_from(21.0_f64) / F::cast_from(512.0_f64) * t27860;
    let t32313 = t32300 - t32302 + t32304 + t32305 / F::cast_from(2.0_f64) + t32307 - t32308 + t32309 - t32310 + t32311 - t32312;
    let t32314 = t739 * t32313;
    (t32313, t32314)
}
