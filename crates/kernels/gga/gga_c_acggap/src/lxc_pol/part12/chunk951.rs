//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 951/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk951<F: Float>(t30219: F, t8515: F, t4680: F, t7575: F, t8514: F, t1181: F, t4930: F, t604: F, t4550: F, t1345: F, t1992: F, t30154: F, t7586: F, t1535: F, t4180: F, t7646: F) -> (F, F, F, F, F, F, F) {
    let t35212 = t30219 * t8515;
    let t35215 = t7575 * t4680 * t8514;
    let t35219 = t7575 * t1181 * t604 * t4930;
    let t35223 = t7575 * t1181 * t604 * t4550;
    let t35225 = t1992 * t1345;
    let t35227 = t30154 * t7586 * t35225;
    let t35230 = t4180 * t7646 * t1535;
    (t35212, t35215, t35219, t35223, t35225, t35227, t35230)
}
