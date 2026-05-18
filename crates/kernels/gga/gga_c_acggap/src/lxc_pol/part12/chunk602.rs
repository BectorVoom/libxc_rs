//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 602/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk602<F: Float>(t1089: F, t1095: F, t4533: F, t1451: F, t997: F, t506: F, t839: F, t368: F, t1077: F, t495: F, t1131: F, t879: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4535 = t1089 * t1095 * t4533;
    let t4538 = t997 * t1451;
    let t4540 = t506 * t839;
    let t4542 = t1089 * t368 * t4540;
    let t4545 = t495 * t1077;
    let t4547 = t1089 * t368 * t4545;
    let t4550 = t495 * t1131;
    let t4552 = t1089 * t1095 * t4550;
    let t4555 = t495 * t879;
    (t4535, t4538, t4540, t4542, t4545, t4547, t4550, t4552, t4555)
}
