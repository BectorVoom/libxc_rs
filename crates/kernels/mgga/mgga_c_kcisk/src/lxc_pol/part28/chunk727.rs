//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 727/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk727<F: Float>(t642: F, t8786: F, t735: F, t734: F, t2576: F, t2587: F, t2591: F, t2563: F, t2567: F, t1935: F, t8973: F, t9017: F, t9021: F, t9023: F, t9025: F, t9027: F, t9031: F, t9033: F) -> (F, F, F, F, F, F, F, F) {
    let t9035 = t642 * t8786;
    let t9036 = t735 * t9035;
    let t9037 = t734 * t9036;
    let t9039 = t2576 * t2587;
    let t9041 = t2576 * t2591;
    let t9043 = t2567 * t2563;
    let t9044 = t1935 * t9043;
    let t9046 = t8973 / 256.0 + t9017 / 16.0 - t9021 / 72.0 + t9023 / 128.0 - t9025 / 3.0 + t9027 / 12.0 - t9031 / 16.0 - t9033 / 8.0 + t9037 / 24.0 + t9039 / 24.0 - t9041 / 96.0 + t9044 / 3.0;
    (t9035, t9036, t9037, t9039, t9041, t9043, t9044, t9046)
}
