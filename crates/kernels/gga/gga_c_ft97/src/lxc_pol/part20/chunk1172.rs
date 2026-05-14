//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1172/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1172<F: Float>(t6875: F, t8232: F, t6154: F, t737: F, t1882: F, t28198: F, t28154: F, t8392: F, t28175: F, t107976: F, t109659: F, t109671: F, t14086: F, t14099: F, t14167: F, t14175: F, t14183: F, t1901: F, t2373: F, t2405: F, t242: F, t24569: F, t2459: F, t2574: F, t2594: F, t28404: F, t446: F, t6947: F, t729: F, t97777: F, t98078: F) -> (F,) {
    let t111512 = t8232 * t6875;
    let t111518 = t737 * t6154;
    let t111523 = 2.0 / 9.0 * t1882 * t28198;
    let t111528 = 2.0 / 27.0 * t8392 * t28154;
    let t111530 = 2.0 / 9.0 * t1882 * t28175;
    let t111531 = -2.0 / 27.0 * t446 * t2594 * t6947 * t2405 - t446 * t242 * t109659 / 3.0 - t446 * t242 * t109671 / 3.0 - t446 * t242 * t107976 / 3.0 - 2.0 / 9.0 * t1901 * t14175 * t24569 * t14086 - 2.0 / 9.0 * t1901 * t97777 * t14167 - 8.0 / 27.0 * t98078 - t446 * t729 * t6947 * t2459 / 3.0 - 4.0 / 27.0 * t111512 + 2.0 / 3.0 * t446 * t2574 * t6947 * t2373 - 4.0 / 9.0 * t1901 * t111518 * t14183 + t111523 + 4.0 / 9.0 * t1901 * t28404 * t14099 - t111528 + t111530;
    (t111531,)
}
