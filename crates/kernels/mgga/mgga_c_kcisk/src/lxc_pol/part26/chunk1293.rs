//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1293/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1293<F: Float>(t33766: F, t9523: F, t33822: F, t4419: F, t2737: F, t9516: F, t32436: F, t33883: F, t32338: F, t9850: F, t32342: F, t33794: F, t33873: F, t9512: F, t32473: F, t9859: F) -> (F, F, F, F, F, F, F, F) {
    let t115535 = t33766 * t9523;
    let t115539 = t4419 * t33822;
    let t115541 = 0.34722222222222222222e-2 * t2737 * t115539;
    let t115550 = 0.13402777777777777778e-2 * t9516 * t115539;
    let t115555 = 0.11574074074074074074e-2 * t32436 * t33883;
    let t115558 = t9850 * t32338;
    let t115566 = 0.11574074074074074074e-2 * t33794 * t32342;
    let t115578 = 0.34722222222222222222e-2 * t9512 * t33873;
    let t115589 = t32473 * t9859;
    (t115535, t115541, t115550, t115555, t115558, t115566, t115578, t115589)
}
