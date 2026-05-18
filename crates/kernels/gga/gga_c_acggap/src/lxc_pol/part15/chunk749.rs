//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 749/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk749<F: Float>(t464: F, t8331: F, t633: F, t864: F, t2132: F, t7885: F, t862: F, t865: F, t103: F, t566: F, t95: F, t1298: F, t469: F) -> (F, F, F, F, F, F, F, F) {
    let t8332 = t8331 * t464;
    let t8336 = t633 * t864;
    let t8337 = t2132 * t8336;
    let t8339 = F::new(0.26020884564615598386e1) * t7885 * t8337;
    let t8347 = t862 * t633;
    let t8349 = F::new(0.13170898365871023197e1) * t8347 * t865;
    let t8372 = t566 * t95 * t103;
    let t8382 = t469 * t1298;
    (t8332, t8336, t8337, t8339, t8347, t8349, t8372, t8382)
}
