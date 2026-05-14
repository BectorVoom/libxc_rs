//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 866/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk866<F: Float>(t1218: F, t1253: F, t14603: F, t14608: F, t14616: F, t14906: F, t15548: F, t2649: F, t2745: F, t2892: F, t317: F, t4027: F, t4135: F, t4309: F, t830: F, t880: F) -> (F,) {
    let t15549 = -t1218 * t2892 - t1253 * t2649 - t1253 * t2745 - t14906 * t317 - 2.0 * t4027 * t880 - 2.0 * t4135 * t880 - 2.0 * t4309 * t830 - 12.0 * t14603 + 4.0 * t14608 + 8.0 * t14616 + t15548;
    (t15549,)
}
