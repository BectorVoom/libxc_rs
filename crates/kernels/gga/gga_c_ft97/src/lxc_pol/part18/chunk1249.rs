//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1249/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1249<F: Float>(t1882: F, t26461: F, t100340: F, t100382: F, t102426: F, t110: F, t11392: F, t11568: F, t11593: F, t11599: F, t11810: F, t11863: F, t1332: F, t1588: F, t1755: F, t1871: F, t1876: F, t1901: F, t1922: F, t23339: F, t25996: F, t26061: F, t26167: F, t26410: F, t379: F, t446: F, t452: F, t47120: F, t488: F, t499: F, t5691: F, t6469: F, t6557: F, t83: F, t8557: F, t91862: F) -> (F,) {
    let t103305 = 2.0 / 9.0 * t1882 * t26461;
    let t103339 = -2.0 / 3.0 * t1901 * t11810 * t23339 * t11568 - 4.0 / 3.0 * t1901 * t47120 * t26167 + t446 * t452 * t488 * t1332 * t11392 / 3.0 + 4.0 / 3.0 * t446 * t1871 * t499 * t25996 + 2.0 / 3.0 * t446 * t1871 * t1922 * t6469 + t103305 + t446 * t452 * t488 * t6557 * t1755 / 3.0 + 2.0 / 3.0 * t446 * t452 * t26061 * t1876 + 2.0 / 3.0 * t446 * t83 * t102426 - 2.0 / 3.0 * t446 * t1871 * t488 * t6557 * t1588 + 2.0 / 27.0 * t91862 - 2.0 / 9.0 * t1901 * t8557 * t26410 * t379 - 8.0 / 9.0 * t11593 * t11863 * t100382 - 4.0 / 9.0 * t11593 * t8557 * t5691 * t11599 + 2.0 / 3.0 * t446 * t1871 * t110 * t100340;
    (t103339,)
}
