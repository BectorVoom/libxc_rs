//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1297/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1297<F: Float>(t106296: F, t106619: F, t11593: F, t119625: F, t119858: F, t12703: F, t13220: F, t16671: F, t16675: F, t16955: F, t16996: F, t17001: F, t17384: F, t1901: F, t23548: F, t23909: F, t26935: F, t27216: F, t27221: F, t30130: F, t30223: F, t30446: F, t30518: F, t379: F, t40792: F, t40945: F, t41269: F, t50229: F, t50744: F, t50773: F, t63855: F, t63863: F, t95789: F) -> (F,) {
    let t120271 = -2.0 / 9.0 * t1901 * t95789 * t17384 - 2.0 / 9.0 * t1901 * t40945 * t30446 - 2.0 / 9.0 * t1901 * t50773 * t26935 - 4.0 / 9.0 * t1901 * t12703 * t119858 + 4.0 / 27.0 * t1901 * t106619 * t16675 + 2.0 / 9.0 * t1901 * t40792 * t30518 * t379 - 4.0 / 9.0 * t1901 * t106296 * t16671 - 4.0 / 27.0 * t1901 * t50744 * t119625 - 2.0 / 27.0 * t1901 * t41269 * t23909 * t16955 - 4.0 / 9.0 * t1901 * t13220 * t30130 * t379 - 4.0 / 9.0 * t1901 * t63855 * t27216 + 4.0 / 27.0 * t1901 * t63863 * t27221 - 4.0 / 9.0 * t1901 * t13220 * t23548 * t16996 - 8.0 / 9.0 * t11593 * t13220 * t23548 * t17001 - 4.0 / 9.0 * t1901 * t50229 * t30223;
    (t120271,)
}
