//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1323/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1323<F: Float>(t1882: F, t30325: F, t160: F, t30105: F, t106934: F, t106940: F, t106957: F, t106958: F, t107012: F, t107019: F, t107022: F, t119452: F, t119482: F, t119505: F, t144: F, t17012: F, t17052: F, t17056: F, t1901: F, t2221: F, t23470: F, t26863: F, t379: F, t446: F) -> (F,) {
    let t121198 = t1882 * t30325;
    let t121212 = t160 * t30105;
    let t121220 = -t106934 - 2.0 * t446 * t144 * t119452 - t106940 + t106957 - 8.0 / 27.0 * t106958 - 2.0 / 9.0 * t121198 + t1901 * t23470 * t17012 / 9.0 + 2.0 / 9.0 * t1901 * t23470 * t17052 - 2.0 / 27.0 * t1901 * t26863 * t17056 - 2.0 / 3.0 * t446 * t144 * t119505 + t1901 * t2221 * t121212 * t379 / 9.0 + 2.0 / 3.0 * t446 * t144 * t119482 - t107012 - t107019 - t107022;
    (t121220,)
}
