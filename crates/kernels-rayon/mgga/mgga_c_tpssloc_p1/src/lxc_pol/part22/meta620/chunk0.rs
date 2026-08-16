//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2151/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2151(t1214: f64, t820: f64, t3624: f64, t52627: f64, t43763: f64, t44827: f64, t3515: f64, t4983: f64, t49850: f64, t11818: f64, t1213: f64, t248: f64, t5012: f64) -> (f64, f64, f64, f64, f64) {
    let t52897 = t820 * t1214;
    let t52903 = t3624 * t52627;
    let t52919 = t44827 * t43763;
    let t52952 = t3515 * t49850 * t4983;
    let t52953 = t52952 / 4608.0_f64;
    let t52973 = t1213 * t248 * t11818 * t5012;
    (t52897, t52903, t52919, t52953, t52973)
}
