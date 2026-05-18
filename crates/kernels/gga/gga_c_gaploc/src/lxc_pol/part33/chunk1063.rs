//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1063/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1063<F: Float>(t2616: F, t486: F, t1890: F, t21783: F, t20157: F, t2085: F, t805: F, t5654: F, t7426: F, t2032: F, t6134: F, t7177: F, t900: F) -> (F, F, F, F, F, F) {
    let t22767 = t486 * t2616;
    let t22775 = t1890 * t21783;
    let t22826 = t805 * t2085 * t20157;
    let t22854 = t5654 * t7426;
    let t22883 = t6134 * t2032;
    let t22909 = t900 * t7177;
    (t22767, t22775, t22826, t22854, t22883, t22909)
}
