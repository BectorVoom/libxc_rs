//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1242/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1242<F: Float>(t100: F, t37429: F, t26177: F, t8392: F, t1882: F, t26326: F, t100370: F, t10964: F, t11443: F, t11490: F, t11496: F, t11520: F, t11568: F, t11593: F, t11613: F, t11618: F, t11810: F, t11863: F, t11902: F, t1901: F, t23245: F, t23249: F, t23323: F, t23340: F, t26166: F, t26172: F, t26210: F, t26372: F, t26373: F, t480: F, t5630: F, t5717: F, t60901: F, t7750: F, t8506: F) -> (F,) {
    let t102948 = t37429 * t100;
    let t102954 = 4.0 / 9.0 * t8392 * t26177;
    let t102960 = 4.0 / 9.0 * t1882 * t26326;
    let t102992 = -2.0 / 3.0 * t1901 * t11490 * t26166 * t10964 - 4.0 * t1901 * t7750 * t480 * t26172 + 8.0 * t1901 * t102948 * t5630 * t11618 + t102954 + 4.0 / 3.0 * t1901 * t11810 * t26166 * t11613 - t102960 - 4.0 / 9.0 * t1901 * t11863 * t100370 - 2.0 * t1901 * t26372 * t26373 * t10964 - 2.0 / 3.0 * t1901 * t11490 * t23249 * t11568 - 4.0 / 3.0 * t1901 * t11810 * t5717 * t11520 - 4.0 / 3.0 * t1901 * t11490 * t23249 * t11496 + 2.0 / 9.0 * t1901 * t11902 * t23245 + 4.0 / 9.0 * t11593 * t8506 * t26210 - 4.0 / 3.0 * t1901 * t60901 * t23340 + t1901 * t23323 * t11443 / 9.0;
    (t102992,)
}
