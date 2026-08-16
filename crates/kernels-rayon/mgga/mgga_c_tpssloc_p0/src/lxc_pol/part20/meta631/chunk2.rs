//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2298/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2298(t13068: f64, t225: f64, t13030: f64, t10046: f64, t10049: f64, t10104: f64, t10111: f64, t10112: f64, t13053: f64, t13065: f64, t13463: f64, t1492: f64, t1527: f64, t1528: f64, t259: f64, t2720: f64, t2743: f64, t40852: f64, t40875: f64, t40890: f64, t4147: f64, t41554: f64, t4268: f64, t4301: f64, t855: f64, t866: f64) -> f64 {
    let t47568 = t13068 * t225;
    let t47585 = t13030 * t225;
    let t47593 = 24.0_f64 * t10111 * t1527 * t40890 * t855 + t10046 * t1492 * t259 - 3.0_f64 * t10049 * t4301 - t10104 * t4268 - 6.0_f64 * t10112 * t4147 - 3.0_f64 * t13053 * t2743 + 6.0_f64 * t13065 * t2720 + 6.0_f64 * t13463 * t2720 - t1528 * t40852 - t1528 * t40875 - 3.0_f64 * t1528 * t41554 - 6.0_f64 * t47568 * t866 - 3.0_f64 * t47585 * t866;
    t47593
}
