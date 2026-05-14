//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 817/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk817<F: Float>(t1530: F, t7432: F, t30153: F, t3360: F, t1101: F, t1992: F, t7842: F, t2070: F, t30792: F, t2067: F, t4198: F, t30267: F, t7643: F, t30225: F, t438: F, t30248: F, t431: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30817 = t1530 * t7432;
    let t30827 = t3360 * t30153;
    let t30830 = t30827 * t7842 * t1992 * t1101;
    let t30854 = t30792 * t2070;
    let t30856 = t4198 * t2067;
    let t30861 = t3360 * t30267;
    let t30862 = t30861 * t7643;
    let t30866 = t30225 * t438;
    let t30868 = t30248 * t431;
    (t30817, t30827, t30830, t30854, t30856, t30861, t30862, t30866, t30868)
}
