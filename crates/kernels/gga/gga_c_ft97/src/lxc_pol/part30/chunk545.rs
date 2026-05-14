//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 545/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk545<F: Float>(t27499: F, t3789: F, t2446: F, t3886: F, t6035: F, t1103: F, t12: F, t14: F, t6056: F, t2247: F, t6044: F, t2917: F, t3746: F, t17859: F, t231: F, t6045: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27500 = t3789 * t27499;
    let t27501 = t2446 * t3886;
    let t27502 = t6035 * t27501;
    let t27505 = t12 * t1103;
    let t27506 = t27505 * t14;
    let t27507 = t27506 * t6056;
    let t27510 = t6044 * t2247;
    let t27511 = t2917 * t3746;
    let t27512 = t27510 * t27511;
    let t27515 = t231 * t17859;
    let t27516 = t6045 * t27515;
    (t27500, t27501, t27502, t27506, t27507, t27511, t27512, t27515, t27516)
}
