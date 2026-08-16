//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1017/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1017(t4551: f64, t4589: f64, t8418: f64, t1852: f64, t20395: f64, t979: f64, t61025: f64, t110: f64, t1866: f64, t1871: f64, t1901: f64, t4436: f64, t4458: f64, t446: f64, t447: f64, t4572: f64, t4623: f64, t75482: f64, t75487: f64, t75489: f64, t75491: f64, t75493: f64, t83: f64, t85531: f64, t85538: f64, t8557: f64) -> (f64, f64, f64, f64) {
    let t85882 = t8418 * t4551 * t4589;
    let t85895 = t1852 * t979 * t20395;
    let t85903 = t61025 * t4551;
    let t85924 = 8.0_f64 / 3.0_f64 * t446 * t83 * t85895 - 2.0_f64 / 9.0_f64 * t446 * t1866 * t110 * t85531 + 4.0_f64 * t446 * t83 * t85903 - 4.0_f64 / 9.0_f64 * t75482 - 8.0_f64 / 27.0_f64 * t75487 + 4.0_f64 * t446 * t1871 * t4623 * t4436 - 4.0_f64 / 3.0_f64 * t75489 + 4.0_f64 / 9.0_f64 * t75491 - 4.0_f64 / 3.0_f64 * t75493 + 8.0_f64 / 3.0_f64 * t446 * t447 * t110 * t85538 + 8.0_f64 / 3.0_f64 * t1901 * t8557 * t4458 * t4572;
    (t85882, t85895, t85903, t85924)
}
