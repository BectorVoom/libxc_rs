//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 902/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk902<F: Float>(t650: F, t8595: F, t1976: F, t7056: F, t6876: F, t8641: F, t2075: F, t6534: F, t652: F, t671: F, t7000: F, t8607: F) -> (F, F, F, F, F, F, F, F) {
    let t31733 = t650 * t8595;
    let t31734 = t1976 * t7056;
    let t31737 = t6876 * t8641;
    let t31744 = t2075 * t6534;
    let t31746 = F::cast_from(2.0_f64) * t652 * t31744;
    let t31747 = t8595 * t671;
    let t31749 = F::cast_from(2.0_f64) * t652 * t31747;
    let t31750 = t8607 * t7000;
    (t31733, t31734, t31737, t31744, t31746, t31747, t31749, t31750)
}
