//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1007/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1007(t285: f64, t6849: f64, t2762: f64, t328: f64, t332: f64, t2315: f64, t286: f64, t2801: f64, t442: f64, t8131: f64, t2254: f64, t8139: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18679 = 1.0_f64 / t6849 / t285;
    let t18680 = 1.0_f64 / t2762 / t328 * t332 * t18679;
    let t18813 = t2315 * t286;
    let t18815 = t8131 * t2801 * t18813 * t442;
    let t18822 = t2254 * t286;
    let t18824 = t8139 * t18822 * t442;
    (t18679, t18680, t18813, t18815, t18822, t18824)
}
