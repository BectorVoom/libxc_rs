//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1400/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1400<F: Float>(t28063: F, t6524: F, t6456: F, t10044: F, t10047: F, t19099: F, t19102: F, t19124: F, t19128: F, t19153: F, t23264: F, t23266: F, t23272: F, t8351: F, t8355: F, t8438: F, t8442: F, t8447: F, t8453: F) -> (F,) {
    let t28188 = t6524 * t28063;
    let t28195 = t6456 * t28063;
    let t28205 = -0.95275595817932748826e-4 * t19099 + 0.47637797908966374413e-4 * t19102 + 0.1270341277572436651e-3 * t19124 + 0.91464571985215438872e-2 * t10044 * t8351 + 0.13719685797782315831e-1 * t28188 * t8438 + 0.45732285992607719436e-2 * t10047 * t8442 + 0.22866142996303859718e-2 * t10047 * t8447 - 0.22866142996303859718e-2 * t28195 * t8453 - 0.45732285992607719436e-2 * t10047 * t8355 + 0.15244095330869239812e-2 * t23264 - 0.91464571985215438872e-2 * t23266 - 0.2540682555144873302e-3 * t23272 + 0.95275595817932748826e-4 * t19128 - 0.2540682555144873302e-3 * t19153;
    (t28205,)
}
