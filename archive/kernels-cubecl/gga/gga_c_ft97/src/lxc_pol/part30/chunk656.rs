//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 656/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk656<F: Float>(t1701: F, t4125: F, t6027: F, t27494: F, t811: F, t820: F, t992: F, t704: F, t25069: F, t4113: F, t22511: F, t7004: F) -> (F, F, F, F, F, F) {
    let t28540 = t1701 * t6027 * t4125;
    let t28544 = t1701 * t27494 * t811;
    let t28547 = t992 * t820;
    let t28548 = t704 * t28547;
    let t28552 = t4113 * t25069;
    let t28557 = t7004 * t22511;
    (t28540, t28544, t28547, t28548, t28552, t28557)
}
