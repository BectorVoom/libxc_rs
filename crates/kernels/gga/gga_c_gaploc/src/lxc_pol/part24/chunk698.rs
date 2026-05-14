//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 698/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk698<F: Float>(t4598: F, t917: F, t1628: F, t2433: F, t569: F, t6393: F, t568: F, t1265: F, t161: F, t165: F) -> (F, F, F, F) {
    let t6876 = t4598 * t917;
    let t6881 = t1628 * t2433;
    let t6888 = t569 * t6393;
    let t6889 = t568 * t6888;
    let t6895 = t161 * t165 * t1265;
    (t6876, t6881, t6889, t6895)
}
