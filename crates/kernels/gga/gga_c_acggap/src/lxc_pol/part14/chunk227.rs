//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 227/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk227<F: Float>(t180: F, t851: F, t323: F, t310: F, t443: F) -> (F, F, F, F) {
    let t852 = t851 * t180;
    let t854 = F::new(0.13170898365871023197e1) * t852 * t323;
    let t855 = t310 * t443;
    let t857 = t310 * t180;
    (t852, t854, t855, t857)
}
