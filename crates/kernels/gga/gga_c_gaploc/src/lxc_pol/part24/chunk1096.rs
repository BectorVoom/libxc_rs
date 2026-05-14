//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1096/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1096<F: Float>(t27837: F, t27840: F, t27844: F, t27848: F, t27853: F, t27856: F, t27858: F, t27860: F, t471: F, t10657: F, t64: F, t3427: F, t90: F, t871: F, t8710: F, t739: F) -> (F, F) {
    let t32300 = (189.0 / 512.0 * t27837 - 2499.0 / 16384.0 * t27840 + 1239.0 / 524288.0 * t27844 - 441.0 / 0.16777216e8 * t27848 + 147.0 / 0.16777216e8 * t27853 - 413.0 / 524288.0 * t27856 + 833.0 / 16384.0 * t27858 - 63.0 / 512.0 * t27860) * t471;
    let t32302 = 8.0 / 3.0 * t10657 * t64;
    let t32304 = 4.0 / 3.0 * t3427 * t90;
    let t32305 = t8710 * t871;
    let t32307 = 63.0 / 512.0 * t27837;
    let t32308 = 385.0 / 16384.0 * t27840;
    let t32309 = 147.0 / 1048576.0 * t27844;
    let t32310 = 49.0 / 1048576.0 * t27856;
    let t32311 = 385.0 / 49152.0 * t27858;
    let t32312 = 21.0 / 512.0 * t27860;
    let t32313 = t32300 - t32302 + t32304 + t32305 / 2.0 + t32307 - t32308 + t32309 - t32310 + t32311 - t32312;
    let t32314 = t739 * t32313;
    (t32313, t32314)
}
