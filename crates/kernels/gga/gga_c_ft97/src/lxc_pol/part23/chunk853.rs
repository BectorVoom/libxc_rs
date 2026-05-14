//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 853/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk853<F: Float>(t27506: F, t6056: F, t2247: F, t6044: F, t2917: F, t3746: F, t17859: F, t231: F, t6045: F, t2426: F, t39: F, t5585: F, t3789: F) -> (F, F, F, F, F, F, F) {
    let t27507 = t27506 * t6056;
    let t27510 = t6044 * t2247;
    let t27511 = t2917 * t3746;
    let t27512 = t27510 * t27511;
    let t27515 = t231 * t17859;
    let t27516 = t6045 * t27515;
    let t27519 = t2426 * t39;
    let t27520 = t27519 * t5585;
    let t27521 = t3789 * t27520;
    (t27507, t27511, t27512, t27515, t27516, t27519, t27521)
}
