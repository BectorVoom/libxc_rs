//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1162/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1162<F: Float>(t7310: F, t8878: F, t1446: F, t7614: F, t2001: F, t4542: F, t1988: F, t8978: F, t1089: F, t1579: F, t2079: F, t2080: F) -> (F, F, F, F, F) {
    let t35924 = t7310 * t8878;
    let t35926 = t7614 * t1446;
    let t35927 = F::new(0.32012600194825403606e-1) * t35926;
    let t35928 = t2001 * t4542;
    let t35930 = t1988 * t8978;
    let t35931 = F::new(0.42874018118069736972e-3) * t35930;
    let t35934 = t2079 * t1089 * t1579 * t2080;
    (t35924, t35927, t35928, t35931, t35934)
}
