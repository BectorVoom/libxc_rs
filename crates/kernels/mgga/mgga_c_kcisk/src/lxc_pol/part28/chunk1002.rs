//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1002/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1002<F: Float>(t8632: F, t970: F, t8620: F, t8623: F, t965: F, t1842: F, t22592: F, t1856: F, t1835: F, t8640: F, t960: F, t8643: F, t8649: F, t8652: F, t1850: F, t7715: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23234 = t970 * t8632;
    let t23236 = t970 * t8620;
    let t23238 = t965 * t8623;
    let t23240 = t1842 * t22592;
    let t23243 = t1856 * t22592;
    let t23246 = t1835 * t22592;
    let t23249 = t960 * t8640;
    let t23251 = t960 * t8643;
    let t23253 = t960 * t8649;
    let t23255 = t965 * t8652;
    let t23259 = t1850 * t7715;
    (t23234, t23236, t23238, t23240, t23243, t23246, t23249, t23251, t23253, t23255, t23259)
}
