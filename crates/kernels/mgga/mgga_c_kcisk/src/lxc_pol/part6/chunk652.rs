//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 652/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk652<F: Float>(t5290: F, t8946: F, t5289: F, t747: F, t8672: F, t746: F, t5315: F, t41: F, t8831: F, t719: F, t734: F, t2567: F, t2571: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9047 = t5290 * t8946;
    let t9048 = t5289 * t9047;
    let t9050 = t747 * t8672;
    let t9051 = t746 * t9050;
    let t9052 = t5315 * t9051;
    let t9054 = t8831 * t41;
    let t9055 = t9054 * t719;
    let t9056 = t734 * t9055;
    let t9058 = t2567 * t2571;
    (t9047, t9048, t9050, t9051, t9052, t9054, t9055, t9056, t9058)
}
