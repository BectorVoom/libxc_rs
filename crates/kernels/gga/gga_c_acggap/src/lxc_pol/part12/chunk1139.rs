//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1139/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1139<F: Float>(t1017: F, t2060: F, t2288: F, t36222: F, t4258: F, t8806: F, t30248: F, t532: F, t537: F, t7637: F, t8859: F, t1576: F, t7614: F) -> (F, F, F, F, F, F) {
    let t36225 = t2060 * t36222 * t2288 * t1017;
    let t36227 = t8806 * t4258;
    let t36231 = t30248 * t532;
    let t36236 = t30248 * t537;
    let t36238 = t7637 * t8859;
    let t36240 = t7614 * t1576;
    (t36225, t36227, t36231, t36236, t36238, t36240)
}
