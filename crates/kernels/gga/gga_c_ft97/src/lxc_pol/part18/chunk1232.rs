//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1232/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1232<F: Float>(t26309: F, t8392: F, t1339: F, t1786: F, t1882: F, t26432: F, t26346: F, t26234: F, t100386: F, t100390: F, t100400: F, t11397: F, t11468: F, t11525: F, t11552: F, t11593: F, t11810: F, t11854: F, t11863: F, t1643: F, t1647: F, t1651: F, t1871: F, t1901: F, t1902: F, t23153: F, t26356: F, t3238: F, t446: F, t47399: F, t5717: F, t6478: F, t6547: F, t8518: F, t8557: F) -> (F,) {
    let t102508 = 4.0 / 27.0 * t8392 * t26309;
    let t102524 = t1786 * t1339;
    let t102533 = 2.0 / 9.0 * t1882 * t26432;
    let t102543 = 4.0 / 27.0 * t8392 * t26346;
    let t102549 = 2.0 / 9.0 * t1882 * t26234;
    let t102550 = -8.0 / 9.0 * t11593 * t11468 * t100386 + 8.0 / 27.0 * t11593 * t11552 * t100390 + t102508 + 2.0 / 9.0 * t1901 * t8557 * t6478 * t1647 + 4.0 / 9.0 * t1901 * t11863 * t100400 - 2.0 / 9.0 * t1901 * t11854 * t6547 * t1651 - 4.0 / 27.0 * t1901 * t47399 * t6547 * t1643 - 4.0 / 9.0 * t1901 * t102524 * t11397 - 2.0 / 3.0 * t446 * t1871 * t3238 * t23153 + t102533 + 2.0 / 27.0 * t1901 * t8518 * t26356 * t1643 - 2.0 / 3.0 * t1901 * t11810 * t5717 * t11525 - t102543 + t1901 * t1902 * t26356 * t1651 / 9.0 - t102549;
    (t102550,)
}
