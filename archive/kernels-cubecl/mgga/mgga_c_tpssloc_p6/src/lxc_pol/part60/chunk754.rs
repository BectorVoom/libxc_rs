//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 754/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk754<F: Float>(t4997: F, t7339: F, t5001: F, t7338: F, t1730: F, t7344: F, t4993: F, t7345: F, t5040: F, t7310: F, t27607: F, t460: F) -> (F, F, F, F, F, F) {
    let t27611 = t7339 * t4997;
    let t27614 = t5001 * t7338;
    let t27617 = t1730 * t7344;
    let t27622 = t7345 * t4993;
    let t27626 = t7310 * t5040;
    let t27628 = t27607 * t460;
    (t27611, t27614, t27617, t27622, t27626, t27628)
}
