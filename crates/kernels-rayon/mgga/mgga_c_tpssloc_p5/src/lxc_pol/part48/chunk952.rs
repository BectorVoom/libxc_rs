//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 952/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk952(t114696: f64, t6552: f64, t6637: f64, t776: f64, t2047: f64, t22986: f64, t6646: f64, t829: f64, t1880: f64, t1894: f64, t214: f64, t24234: f64) -> (f64, f64, f64) {
    let t114699 = t6552 * t6637 * t114696 * t776;
    let t114704 = t22986 * t6646 * t2047 * t776 * t829;
    let t114708 = t1880 * t214 * t1894 * t24234;
    (t114699, t114704, t114708)
}
