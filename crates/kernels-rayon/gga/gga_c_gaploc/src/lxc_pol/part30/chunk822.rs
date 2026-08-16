//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 822/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk822(t6507: f64, t7893: f64, t161: f64, t2760: f64, t1353: f64, t1359: f64, t3394: f64, t488: f64, t447: f64, t986: f64) -> (f64, f64, f64, f64, f64) {
    let t7894 = t6507 * t7893;
    let t7897 = t2760 * t161;
    let t7898 = t7897 * t1353;
    let t7901 = t1359 * t3394;
    let t7902 = t7901 * t488;
    let t7905 = t986 * t447;
    (t7894, t7898, t7901, t7902, t7905)
}
