//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 960/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk960<F: Float>(t1967: F, t8536: F, t30543: F, t8661: F, t30219: F, t8610: F, t30937: F, t8614: F, t30934: F, t8597: F, t2264: F, t30797: F) -> (F, F, F, F, F, F) {
    let t34013 = t1967 * t8536;
    let t34023 = t30543 * t8661;
    let t34027 = t30219 * t8610;
    let t34029 = t30937 * t8614;
    let t34031 = t30934 * t8597;
    let t34033 = t30797 * t2264;
    (t34013, t34023, t34027, t34029, t34031, t34033)
}
