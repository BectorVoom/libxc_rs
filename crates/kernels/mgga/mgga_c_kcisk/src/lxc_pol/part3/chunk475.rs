//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 475/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk475<F: Float>(t1341: F, t3732: F, t1415: F, t1411: F, t10: F, t79: F) -> (F, F, F, F) {
    let t3733 = t1341 * t3732;
    let t3734 = t1415 * t3733;
    let t3735 = t1411 * t3734;
    let t3737 = t10 * t79;
    (t3733, t3734, t3735, t3737)
}
