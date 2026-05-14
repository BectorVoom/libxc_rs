//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 867/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk867<F: Float>(t26062: F, t83: F, t26059: F, t1871: F, t499: F, t6469: F, t110: F, t25996: F, t103: F, t26041: F, t82: F, t26114: F, t452: F, t5617: F, t986: F, t23311: F, t23312: F, t23319: F, t23321: F, t23344: F, t23358: F, t23360: F, t28: F, t446: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t26461 = t83 * t26062;
    let t26464 = t83 * t26059;
    let t26468 = t1871 * t499 * t6469;
    let t26472 = t1871 * t110 * t25996;
    let t26476 = t82 * t26041 * t103;
    let t26480 = t83 * t26114;
    let t26487 = t452 * t986 * t5617;
    let t26490 = -t23311 + t23312 / 9.0 + t23319 / 9.0 + t23321 / 9.0 - t446 * t26461 / 3.0 - t446 * t26464 / 3.0 + 2.0 / 3.0 * t446 * t26468 + 2.0 / 3.0 * t446 * t26472 + t89 * t28 * t26476 / 3.0 - t446 * t26480 / 3.0 - t23344 / 27.0 + t23358 / 9.0 + t23360 / 9.0 - t446 * t26487 / 3.0;
    (t26461, t26464, t26468, t26472, t26476, t26480, t26487, t26490)
}
