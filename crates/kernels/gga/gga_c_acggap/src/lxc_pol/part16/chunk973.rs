//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 973/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk973<F: Float>(t2264: F, t30456: F, t1165: F, t30209: F, t5099: F, t7351: F, t30546: F, t8657: F, t4198: F, t7646: F, t1061: F, t535: F) -> (F, F, F, F, F) {
    let t34468 = t30456 * t2264;
    let t34476 = t30209 * t1165 * t7351 * t5099;
    let t34478 = t30546 * t8657;
    let t34481 = t4198 * t7646;
    let t34487 = t535 * t1061;
    (t34468, t34476, t34478, t34481, t34487)
}
