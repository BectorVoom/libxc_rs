//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 316/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk316<F: Float>(t1440: F, t499: F, t498: F, t1504: F, t390: F, t494: F, t391: F, t79: F) -> (F, F, F, F, F) {
    let t1505 = t499 * t1440;
    let t1506 = t498 * t1505;
    let t1507 = t1504 * t1506;
    let t1509 = t494 * t390;
    let t1511 = F::new(1.0) / t391 / t1509;
    let t1512 = t1511 * t79;
    (t1505, t1506, t1507, t1511, t1512)
}
