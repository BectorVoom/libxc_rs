//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 801/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk801<F: Float>(t2274: F, t7315: F, t2016: F, t2278: F, t500: F, t7329: F, t1462: F, t2001: F, t1089: F, t2080: F, t535: F, t2079: F) -> (F, F, F, F, F, F) {
    let t8680 = t7315 * t2274;
    let t8682 = t2016 * t2278;
    let t8684 = t7329 * t500;
    let t8686 = t2001 * t1462;
    let t8689 = t1089 * t535 * t2080;
    let t8690 = t2079 * t8689;
    (t8680, t8682, t8684, t8686, t8689, t8690)
}
