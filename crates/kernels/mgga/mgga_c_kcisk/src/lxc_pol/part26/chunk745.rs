//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 745/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk745<F: Float>(t1520: F, t9483: F, t2732: F, t4165: F, t4170: F, t1482: F, t488: F, t1486: F, t394: F) -> (F, F, F, F, F, F) {
    let t9484 = t9483 * t1520;
    let t9485 = t4165 * t2732;
    let t9486 = t2732 * t1520;
    let t9488 = 2.0 * t4170 * t9486;
    let t9489 = t1482 * t488;
    let t9491 = t1486 * t394;
    (t9484, t9485, t9486, t9488, t9489, t9491)
}
