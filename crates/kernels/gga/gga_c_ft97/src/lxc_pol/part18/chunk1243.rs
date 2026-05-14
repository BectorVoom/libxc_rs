//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1243/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1243<F: Float>(t26185: F, t8392: F, t1882: F, t26464: F, t6559: F, t8232: F, t26387: F, t1852: F, t23084: F, t979: F, t6544: F, t11854: F, t11906: F, t11993: F, t12004: F, t1339: F, t1755: F, t1876: F, t1901: F, t23231: F, t23327: F, t25590: F, t3281: F, t358: F, t379: F, t446: F, t447: F, t452: F, t60426: F, t6478: F, t6564: F, t83: F, t91705: F, t91718: F, t92035: F) -> (F, F) {
    let t102997 = 2.0 / 27.0 * t8392 * t26185;
    let t102999 = 2.0 / 9.0 * t1882 * t26464;
    let t103010 = t8232 * t6559;
    let t103013 = 2.0 / 9.0 * t1882 * t26387;
    let t103015 = t1852 * t23084 * t979;
    let t103029 = t8232 * t6544;
    let t103035 = t1901 * t11906 * t23231 / 9.0 - t102997 + t102999 + t1901 * t23327 * t11993 / 9.0 + 2.0 / 27.0 * t1901 * t92035 * t12004 + 8.0 / 3.0 * t1901 * t60426 * t6478 * t1876 - 4.0 / 27.0 * t103010 + t103013 + 2.0 / 3.0 * t446 * t83 * t103015 - 4.0 / 9.0 * t1901 * t11854 * t25590 * t379 - t446 * t452 * t6564 * t1755 / 3.0 + 16.0 / 27.0 * t91705 + 8.0 / 27.0 * t91718 - 4.0 / 27.0 * t103029 - 2.0 / 9.0 * t3281 * t447 * t1339 * t358;
    (t103015, t103035)
}
