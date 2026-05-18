//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 892/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk892<F: Float>(t226: F, t5903: F, t1640: F, t1791: F, t187: F, t190: F, t367: F, t16704: F, t1764: F, t177: F, t191: F, t5463: F, t649: F) -> (F, F, F, F, F, F) {
    let t17552 = F::new(16.0) / F::new(3.0) * t226 * t5903;
    let t17646 = t1640 * t1791;
    let t17678 = F::new(0.10864197530864197531e0) * t190 * t367 * t187;
    let t17728 = F::new(0.37324691358024691357e0) * t16704;
    let t17758 = t191 / t177 / t1764;
    let t17791 = t5463 * t649;
    (t17552, t17646, t17678, t17728, t17758, t17791)
}
