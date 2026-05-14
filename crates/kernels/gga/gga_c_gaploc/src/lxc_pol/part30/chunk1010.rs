//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1010/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1010<F: Float>(t2549: F, t9625: F, t1949: F, t3240: F, t731: F, t9630: F, t21483: F, t2562: F, t883: F, t943: F, t2558: F, t7589: F, t2537: F, t7064: F, t7177: F, t1842: F, t21491: F) -> (F, F, F, F, F, F, F) {
    let t29212 = 0.1281754371690370714e-2 * t2549 * t9625;
    let t29224 = 0.17090058289204942853e-2 * t1949 * t3240;
    let t29226 = 0.17090058289204942853e-2 * t731 * t9630;
    let t29230 = 0.64087718584518535698e-3 * t943 * t2562 * t883 * t21483;
    let t29233 = 0.64087718584518535698e-3 * t943 * t7589 * t2558;
    let t29242 = 0.64087718584518535698e-3 * t7064 * t2537 * t7177;
    let t29273 = 0.3845263115071112142e-2 * t7064 * t1842 * t883 * t21491;
    (t29212, t29224, t29226, t29230, t29233, t29242, t29273)
}
