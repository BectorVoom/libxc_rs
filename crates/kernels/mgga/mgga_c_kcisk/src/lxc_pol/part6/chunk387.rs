//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 387/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk387<F: Float>(t776: F, t2399: F, t41: F, t2442: F, t525: F, t642: F, t773: F, t79: F, t781: F) -> (F, F, F) {
    let t777 = t776 < -0.66725e-1;
    let t2620 = t2399 * t41;
    let t2628 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t2620 * t642 - 10.0 / 27.0 * t525 * t773 * t2442);
    let t2629 = t79 * t2628;
    let t2630 = t2629 * t781;
    (t2620, t2629, t2630)
}
