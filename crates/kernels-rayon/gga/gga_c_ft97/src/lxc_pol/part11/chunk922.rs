//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 922/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk922(t1882: f64, t8555: f64, t1863: f64, t8232: f64, t487: f64, t7763: f64, t100: f64, t38477: f64, t103: f64, t110: f64, t11987: f64, t1588: f64, t1755: f64, t1852: f64, t1853: f64, t1871: f64, t1901: f64, t379: f64, t38662: f64, t38937: f64, t432: f64, t446: f64, t447: f64, t452: f64, t488: f64, t492: f64, t499: f64, t7751: f64, t7966: f64, t8183: f64, t83: f64, t8411: f64, t8466: f64, t8562: f64) -> f64 {
    let t39013 = t1882 * t8555;
    let t39019 = t8232 * t1863;
    let t39021 = t487 * t7763;
    let t39026 = t38477 * t100;
    let t39047 = -8.0_f64 * t446 * t452 * t8466 * t8562 + 8.0_f64 * t446 * t83 * t38662 - 4.0_f64 * t446 * t452 * t1852 * t1755 * t1853 - 4.0_f64 / 3.0_f64 * t39013 - 8.0_f64 / 3.0_f64 * t446 * t447 * t499 * t7966 - 8.0_f64 / 27.0_f64 * t39019 + 40.0_f64 / 81.0_f64 * t1901 * t11987 * t39021 * t38937 + 8.0_f64 / 3.0_f64 * t1901 * t39026 * t103 * t7751 * t379 + 8.0_f64 / 3.0_f64 * t446 * t1871 * t110 * t432 * t8183 + 8.0_f64 * t446 * t8411 * t488 * t7751 * t492 - 12.0_f64 * t446 * t8411 * t110 * t1588 * t1755;
    t39047
}
