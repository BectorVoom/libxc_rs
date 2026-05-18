//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1168/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1168<F: Float>(t10856: F, t9423: F, t11643: F, t25983: F, t261: F, t3304: F, t9476: F, t37982: F, t9373: F, t11654: F, t7601: F, t10743: F, t3198: F) -> (F, F, F, F, F, F) {
    let t43217 = t10856 * t9423;
    let t43219 = t25983 * t11643;
    let t43225 = t3304 * t261 * t9476;
    let t43230 = t37982 * t9373;
    let t43232 = t7601 * t11654;
    let t43234 = t10743 * t3198;
    (t43217, t43219, t43225, t43230, t43232, t43234)
}
