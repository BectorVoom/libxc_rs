//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 778/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk778<F: Float>(t3813: F, t770: F, t7451: F, t2606: F, t2669: F, t3625: F, t2641: F, t2644: F, t3814: F, t896: F, t9: F, t769: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7452 = t3813 * t770;
    let t7453 = t7451 * t7452;
    let t7456 = t2669 * t2606;
    let t7457 = t7456 * t3625;
    let t7460 = t2641 * t2606;
    let t7461 = t7460 * t2644;
    let t7464 = t7456 * t3814;
    let t7467 = t9 * t896;
    let t7468 = t7467 * t769;
    (t7452, t7453, t7456, t7457, t7460, t7461, t7464, t7467, t7468)
}
