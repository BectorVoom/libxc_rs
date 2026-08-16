//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 859/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk859<F: Float>(t1890: F, t8502: F, t590: F, t1392: F, t2949: F, t1391: F, t1835: F, t1445: F, t1980: F, t2975: F, t2925: F, t296: F) -> (F, F, F, F, F, F) {
    let t8503 = t1890 * t8502;
    let t8504 = t8503 * t590;
    let t8508 = t1392 * t2949;
    let t8509 = t1391 * t8508;
    let t8512 = t2949 * t1835;
    let t8513 = t1445 * t8512;
    let t8516 = t1980 * t2975;
    let t8519 = t296 * t2925;
    (t8504, t8509, t8512, t8513, t8516, t8519)
}
