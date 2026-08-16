//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1028/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1028(t7614: f64, t968: f64, t1920: f64, t1948: f64, t4657: f64, t345: f64, t4677: f64, t6800: f64, t6799: f64, t4680: f64, t1022: f64, t1409: f64) -> (f64, f64, f64, f64, f64) {
    let t25529 = t968 * t7614;
    let t25530 = t1920 * t25529;
    let t25535 = t1948 * t4657;
    let t25536 = t345 * t25535;
    let t25540 = t4677 * t6800;
    let t25541 = t6799 * t25540;
    let t25544 = t4680 * t6800;
    let t25545 = t6799 * t25544;
    let t25548 = t1409 * t1022;
    (t25530, t25536, t25541, t25545, t25548)
}
