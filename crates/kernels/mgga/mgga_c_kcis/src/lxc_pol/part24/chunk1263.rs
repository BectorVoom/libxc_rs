//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1263/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1263<F: Float>(t100129: F, t7772: F, t19309: F, t26772: F, t303: F, t1020: F, t26671: F, t28915: F, t27836: F, t27845: F, t4994: F, t26753: F, t28907: F) -> (F, F, F, F, F) {
    let t100656 = t7772 * t100129;
    let t100660 = t303 * t26772 * t19309;
    let t100666 = t1020 * t26671 * t28915;
    let t100669 = t4994 * t27836 * t27845;
    let t100672 = t1020 * t26753 * t28907;
    (t100656, t100660, t100666, t100669, t100672)
}
