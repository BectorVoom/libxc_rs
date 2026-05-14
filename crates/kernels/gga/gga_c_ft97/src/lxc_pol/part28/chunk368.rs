//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 368/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk368<F: Float>(t139: F, t5790: F, t1701: F, t554: F, t5546: F, t527: F, t5784: F, t549: F, t6: F, t8: F, t2001: F) -> (F, F, F, F, F, F) {
    let t5791 = t5790 * t139;
    let t5797 = t1701 * t5546 * t554;
    let t5802 = t527 * t5784;
    let t5811 = t549 * t6;
    let t5812 = t5811 * t8;
    let t5813 = t2001 * t5812;
    (t5791, t5797, t5802, t5811, t5812, t5813)
}
