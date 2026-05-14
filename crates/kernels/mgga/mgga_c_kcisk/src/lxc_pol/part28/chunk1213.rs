//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1213/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1213<F: Float>(t18744: F, t79: F, t2803: F, t9732: F, t9990: F, t10000: F, t9736: F, t4419: F, t9994: F) -> (F, F, F, F, F) {
    let t34473 = t18744 * t79;
    let t34474 = t34473 * t2803;
    let t34477 = t9990 * t9732;
    let t34480 = t10000 * t9736;
    let t34484 = t4419 * t9994;
    (t34473, t34474, t34477, t34480, t34484)
}
