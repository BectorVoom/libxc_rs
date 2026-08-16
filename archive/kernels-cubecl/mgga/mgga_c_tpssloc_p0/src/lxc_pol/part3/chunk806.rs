//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 806/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk806<F: Float>(t3240: F, t4724: F, t123: F, t1409: F, t3247: F, t607: F) -> (F, F, F, F) {
    let t4725 = t3240 * t4724;
    let t4726 = t123 * t4725;
    let t4728 = t3247 * t1409;
    let t4729 = t4728 * t607;
    (t4725, t4726, t4728, t4729)
}
