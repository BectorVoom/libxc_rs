//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 921/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk921<F: Float>(t21101: F, t21148: F, t21467: F, t21497: F, t1386: F, t17292: F, t5637: F, t4160: F, t1307: F, t7313: F, t4170: F, t17298: F, t5668: F) -> (F, F, F, F, F, F) {
    let t21499 = t21101 + t21148 + t21467 + t21497;
    let t21500 = t21499 * t1386;
    let t21507 = t17292 * t5637;
    let t21508 = t4160 * t21507;
    let t21510 = t7313 * t1307;
    let t21511 = t4170 * t21510;
    let t21512 = t4160 * t21511;
    let t21514 = t17298 * t5668;
    (t21499, t21500, t21508, t21510, t21512, t21514)
}
