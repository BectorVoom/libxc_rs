//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1120/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1120<F: Float>(t2095: F, t33901: F, t33884: F, t1998: F, t4503: F, t5124: F, t7647: F, t7310: F, t8878: F, t1446: F, t7614: F, t2001: F, t4542: F) -> (F, F, F, F, F, F, F) {
    let t35913 = t2095 * t33901;
    let t35915 = t2095 * t33884;
    let t35917 = t1998 * t4503;
    let t35919 = t7647 * t5124;
    let t35924 = t7310 * t8878;
    let t35926 = t7614 * t1446;
    let t35928 = t2001 * t4542;
    (t35913, t35915, t35917, t35919, t35924, t35926, t35928)
}
