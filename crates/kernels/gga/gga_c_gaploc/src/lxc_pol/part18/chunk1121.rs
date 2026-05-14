//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1121/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1121<F: Float>(t16136: F, t3504: F, t28387: F, t3025: F, t32214: F, t701: F, t10627: F, t1865: F) -> (F, F, F, F) {
    let t32785 = 0.69017266717057349418e1 * t16136 * t3504;
    let t32791 = 0.10725146985555128001e1 * t3025 * t28387;
    let t32796 = t32214 * t701;
    let t32803 = t10627 * t1865;
    (t32785, t32791, t32796, t32803)
}
