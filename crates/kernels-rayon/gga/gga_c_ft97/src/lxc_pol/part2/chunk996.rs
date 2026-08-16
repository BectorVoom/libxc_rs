//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 996/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk996(t15131: f64, t296: f64, t18: f64, t875: f64, t2882: f64, t2881: f64, t14116: f64, t4265: f64, t10443: f64, t4256: f64, t10730: f64, t10732: f64, t11593: f64, t15404: f64, t15409: f64, t15415: f64, t15419: f64, t15420: f64, t15422: f64, t15427: f64, t15430: f64, t15435: f64, t1901: f64, t446: f64) -> f64 {
    let t15438 = t296 * t15131;
    let t15441 = t18 * t875;
    let t15442 = t2882 * t15441;
    let t15443 = t2881 * t15442;
    let t15446 = t4265 * t14116;
    let t15447 = t2881 * t15446;
    let t15450 = t10443 * t4256;
    let t15453 = 4.0_f64 / 9.0_f64 * t1901 * t15404 - 4.0_f64 / 9.0_f64 * t11593 * t15409 - 2.0_f64 / 9.0_f64 * t10730 - 8.0_f64 / 81.0_f64 * t10732 - 2.0_f64 / 3.0_f64 * t446 * t15415 + t15419 - 4.0_f64 / 27.0_f64 * t15420 - t446 * t15422 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t15427 + 2.0_f64 / 3.0_f64 * t446 * t15430 + t446 * t15435 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t15438 - 4.0_f64 / 9.0_f64 * t11593 * t15443 - 8.0_f64 / 9.0_f64 * t11593 * t15447 + 2.0_f64 / 9.0_f64 * t1901 * t15450;
    t15453
}
