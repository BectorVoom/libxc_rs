//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 169/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk169<F: Float>(t525: F, t642: F, t773: F, t79: F) -> (F, F) {
    let t776 = 10.0 / 9.0 * t525 * t773 * t642;
    let t777 = t776 < -0.66725e-1;
    let t779 = piecewise3(t777, 0.0, 0.66725e-1 + t776);
    let t780 = t79 * t779;
    (t779, t780)
}
