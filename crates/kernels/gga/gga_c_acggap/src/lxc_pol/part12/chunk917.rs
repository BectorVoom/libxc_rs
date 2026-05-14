//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 917/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk917<F: Float>(t2450: F, t31349: F, t4469: F, t7839: F, t8481: F, t2020: F, t8942: F, t5164: F, t7450: F, t7815: F, t2060: F, t5170: F, t1988: F, t8536: F, t2278: F, t7600: F) -> (F, F, F, F, F, F, F, F) {
    let t34406 = t2450 * t31349;
    let t34407 = t34406 * t4469;
    let t34409 = t7839 * t8481;
    let t34421 = t2020 * t8942;
    let t34424 = t7450 * t7815 * t5164;
    let t34427 = t2060 * t7815 * t5170;
    let t34429 = t1988 * t8536;
    let t34433 = t7600 * t2278;
    (t34406, t34407, t34409, t34421, t34424, t34427, t34429, t34433)
}
