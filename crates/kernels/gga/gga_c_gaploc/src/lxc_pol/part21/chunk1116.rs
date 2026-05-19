//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1116/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1116<F: Float>(t21446: F, t739: F, t3248: F, t7211: F, t2549: F, t9625: F, t1949: F, t3240: F, t731: F, t9630: F, t21483: F, t2562: F, t883: F, t943: F) -> (F, F, F, F, F, F) {
    let t29194 = t739 * t21446;
    let t29210 = F::cast_from(0.64087718584518535698e-3_f64) * t7211 * t3248;
    let t29212 = F::cast_from(0.1281754371690370714e-2_f64) * t2549 * t9625;
    let t29224 = F::cast_from(0.17090058289204942853e-2_f64) * t1949 * t3240;
    let t29226 = F::cast_from(0.17090058289204942853e-2_f64) * t731 * t9630;
    let t29230 = F::cast_from(0.64087718584518535698e-3_f64) * t943 * t2562 * t883 * t21483;
    (t29194, t29210, t29212, t29224, t29226, t29230)
}
