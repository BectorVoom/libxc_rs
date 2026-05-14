//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1012/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1012<F: Float>(t21455: F, t739: F, t21446: F, t3248: F, t7211: F, t2549: F, t9625: F, t1949: F, t3240: F, t731: F, t9630: F, t21483: F, t2562: F, t883: F, t943: F, t2558: F, t7589: F) -> (F, F, F, F, F, F, F, F) {
    let t29190 = t739 * t21455;
    let t29194 = t739 * t21446;
    let t29210 = 0.64087718584518535698e-3 * t7211 * t3248;
    let t29212 = 0.1281754371690370714e-2 * t2549 * t9625;
    let t29224 = 0.17090058289204942853e-2 * t1949 * t3240;
    let t29226 = 0.17090058289204942853e-2 * t731 * t9630;
    let t29230 = 0.64087718584518535698e-3 * t943 * t2562 * t883 * t21483;
    let t29233 = 0.64087718584518535698e-3 * t943 * t7589 * t2558;
    (t29190, t29194, t29210, t29212, t29224, t29226, t29230, t29233)
}
