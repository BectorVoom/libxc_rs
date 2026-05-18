//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 541/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk541<F: Float>(t443: F, t848: F, t463: F, t879: F, t449: F, t316: F, t180: F, t464: F, t310: F, t441: F, t852: F, t880: F) -> (F, F, F, F, F, F, F) {
    let t3880 = t848 * t443;
    let t3888 = t879 * t463;
    let t3889 = t449 * t3888;
    let t3890 = t316 * t3889;
    let t3892 = t848 * t180;
    let t3893 = t3892 * t464;
    let t3896 = t310 * t441;
    let t3900 = F::new(0.19756347548806534796e1) * t852 * t880;
    (t3880, t3889, t3890, t3892, t3893, t3896, t3900)
}
