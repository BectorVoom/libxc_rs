//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 866/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk866<F: Float>(t110: F, t22862: F, t452: F, t499: F, t5617: F, t1339: F, t1647: F, t447: F, t1307: F, t1853: F, t1852: F, t23018: F, t8411: F, t1901: F, t23319: F, t23321: F, t23324: F, t23328: F, t23332: F, t23336: F, t23341: F, t23344: F, t23346: F, t23350: F, t23355: F, t23358: F, t23360: F, t446: F) -> (F, F, F, F, F, F, F) {
    let t23363 = t452 * t110 * t22862;
    let t23367 = t452 * t499 * t5617;
    let t23371 = t447 * t1339 * t1647;
    let t23374 = t1307 * t1853;
    let t23376 = t452 * t1852 * t23374;
    let t23380 = t8411 * t110 * t23018;
    let t23383 = 2.0 / 9.0 * t23319 + 2.0 / 9.0 * t23321 + 2.0 / 9.0 * t1901 * t23324 + 2.0 / 9.0 * t1901 * t23328 + t1901 * t23332 / 9.0 + 2.0 / 27.0 * t1901 * t23336 - 4.0 / 3.0 * t1901 * t23341 - 2.0 / 27.0 * t23344 + 2.0 / 9.0 * t1901 * t23346 + 2.0 / 3.0 * t446 * t23350 + 2.0 / 3.0 * t446 * t23355 + 2.0 / 9.0 * t23358 + 2.0 / 9.0 * t23360 - t446 * t23363 / 3.0 - 2.0 / 3.0 * t446 * t23367 + 2.0 / 9.0 * t446 * t23371 - 2.0 / 3.0 * t446 * t23376 - 2.0 * t446 * t23380;
    (t23363, t23367, t23371, t23374, t23376, t23380, t23383)
}
