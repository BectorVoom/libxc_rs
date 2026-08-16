//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 844/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk844<F: Float>(t21655: F, t4265: F, t2881: F, t1091: F, t5424: F, t835: F, t1248: F, t5393: F, t2843: F, t296: F, t1212: F, t5309: F) -> (F, F, F, F, F, F, F) {
    let t22397 = t4265 * t21655;
    let t22398 = t2881 * t22397;
    let t22402 = t835 * t5424 * t1091;
    let t22405 = t1248 * t5393;
    let t22406 = t2843 * t22405;
    let t22407 = t296 * t22406;
    let t22410 = t5309 * t1212;
    (t22397, t22398, t22402, t22405, t22406, t22407, t22410)
}
