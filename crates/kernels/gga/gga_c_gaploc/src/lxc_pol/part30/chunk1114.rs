//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1114/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1114<F: Float>(t21446: F, t739: F, t3248: F, t7211: F, t2549: F, t9625: F, t1949: F, t3240: F, t731: F, t9630: F, t21483: F, t2562: F, t883: F, t943: F) -> (F, F, F, F, F, F) {
    let t29194 = t739 * t21446;
    let t29210 = F::new(0.64087718584518535698e-3) * t7211 * t3248;
    let t29212 = F::new(0.1281754371690370714e-2) * t2549 * t9625;
    let t29224 = F::new(0.17090058289204942853e-2) * t1949 * t3240;
    let t29226 = F::new(0.17090058289204942853e-2) * t731 * t9630;
    let t29230 = F::new(0.64087718584518535698e-3) * t943 * t2562 * t883 * t21483;
    (t29194, t29210, t29212, t29224, t29226, t29230)
}
