//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 489/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk489<F: Float>(t3386: F, t603: F, t1245: F, t539: F, t544: F, t1244: F, t591: F) -> (F, F, F, F) {
    let t3387 = t3386 * t603;
    let t3389 = t539 * t1245;
    let t3391 = t544 * t1245;
    let t3399 = t1244 * t591;
    (t3387, t3389, t3391, t3399)
}
