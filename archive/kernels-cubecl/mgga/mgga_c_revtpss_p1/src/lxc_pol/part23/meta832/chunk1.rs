//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2694/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2694<F: Float>(t1071: F, t6235: F, t6343: F, t989: F, t20230: F, t3336: F, t2435: F, t6430: F) -> (F, F, F, F) {
    let t68185 = t6235 * t1071;
    let t68188 = t989 * t6343;
    let t68207 = t20230 * t3336;
    let t68255 = t2435 * t6430;
    (t68185, t68188, t68207, t68255)
}
