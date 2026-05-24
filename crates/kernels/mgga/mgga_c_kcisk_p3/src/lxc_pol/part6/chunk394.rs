//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 394/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk394<F: Float>(t2576: F, t748: F, t2527: F, t747: F, t746: F, t1948: F, t650: F, t742: F, t651: F, t79: F) -> (F, F, F, F, F, F) {
    let t2577 = t2576 * t748;
    let t2579 = t747 * t2527;
    let t2580 = t746 * t2579;
    let t2581 = t1948 * t2580;
    let t2583 = t742 * t650;
    let t2585 = F::new(1.0) / t651 / t2583;
    let t2586 = t2585 * t79;
    (t2577, t2579, t2580, t2581, t2585, t2586)
}
