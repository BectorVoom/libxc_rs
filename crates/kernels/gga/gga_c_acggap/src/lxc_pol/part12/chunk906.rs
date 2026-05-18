//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 906/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk906<F: Float>(t3178: F, t7647: F, t3171: F, t7361: F, t7839: F, t1145: F, t7329: F, t1117: F, t1103: F, t7736: F, t1089: F, t429: F, t7553: F, t7554: F) -> (F, F, F, F, F, F, F) {
    let t30750 = t7647 * t3178;
    let t30756 = t7647 * t3171;
    let t30758 = t7839 * t7361;
    let t30763 = t7329 * t1145;
    let t30767 = t7329 * t1117;
    let t30769 = t7736 * t1103;
    let t30773 = t7553 * t1089 * t429 * t7554;
    (t30750, t30756, t30758, t30763, t30767, t30769, t30773)
}
