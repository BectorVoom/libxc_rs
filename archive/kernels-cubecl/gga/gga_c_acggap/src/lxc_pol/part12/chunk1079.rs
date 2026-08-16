//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1079/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1079<F: Float>(t1967: F, t8549: F, t30219: F, t8515: F, t4680: F, t7575: F, t8514: F, t1181: F, t4930: F, t604: F, t4550: F, t1345: F, t1992: F) -> (F, F, F, F, F, F) {
    let t35210 = t1967 * t8549;
    let t35212 = t30219 * t8515;
    let t35215 = t7575 * t4680 * t8514;
    let t35219 = t7575 * t1181 * t604 * t4930;
    let t35223 = t7575 * t1181 * t604 * t4550;
    let t35225 = t1992 * t1345;
    (t35210, t35212, t35215, t35219, t35223, t35225)
}
