//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2558/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2558<F: Float>(t3070: F, t43198: F, t4578: F, t4574: F, t14192: F, t2960: F, t10510: F, t4641: F, t1020: F, t1616: F, t248: F, t43216: F) -> (F, F, F, F, F) {
    let t50147 = t3070 * t43198 * t4578;
    let t50169 = t3070 * t43198 * t4574;
    let t50172 = t2960 * t14192;
    let t50174 = t4641 * t10510;
    let t50181 = t1020 * t248 * t43216 * t1616;
    (t50147, t50169, t50172, t50174, t50181)
}
