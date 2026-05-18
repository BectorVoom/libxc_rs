//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 772/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk772<F: Float>(t500: F, t7329: F, t1462: F, t2001: F, t1089: F, t2080: F, t535: F, t2079: F, t1451: F, t1434: F, t1998: F, t1441: F) -> (F, F, F, F, F, F, F) {
    let t8684 = t7329 * t500;
    let t8686 = t2001 * t1462;
    let t8689 = t1089 * t535 * t2080;
    let t8690 = t2079 * t8689;
    let t8692 = t2001 * t1451;
    let t8694 = t1998 * t1434;
    let t8696 = t2001 * t1441;
    (t8684, t8686, t8689, t8690, t8692, t8694, t8696)
}
