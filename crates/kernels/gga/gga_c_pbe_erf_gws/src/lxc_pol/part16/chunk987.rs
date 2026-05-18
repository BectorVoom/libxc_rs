//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 987/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk987<F: Float>(t2409: F, t831: F, t8804: F, t3214: F, t4414: F, t2410: F, t8589: F, t1164: F, t2242: F, t3123: F, t6180: F, t6184: F) -> (F, F, F, F, F, F) {
    let t8806 = t2409 * t831 * t8804;
    let t8810 = F::new(7.0) / F::new(72.0) * t4414 * t3214;
    let t8812 = t2409 * t8589 * t2410;
    let t8818 = t2242 * t1164;
    let t8821 = t3123 * t6180 / F::new(96.0);
    let t8823 = F::new(7.0) / F::new(144.0) * t3123 * t6184;
    (t8806, t8810, t8812, t8818, t8821, t8823)
}
