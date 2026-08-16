//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 757/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk757(t1855: f64, t3053: f64, t3057: f64, t3065: f64, t3121: f64, t1971: f64, t3707: f64, t1030: f64, t3076: f64, t1795: f64, t3104: f64, t1636: f64, t189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8980 = t3053 * t1855;
    let t8982 = t3057 * t1855;
    let t8984 = t3121 * t3065;
    let t8986 = t1971 * t3707;
    let t8987 = t1030 * t8986;
    let t8988 = t8987 * t3076;
    let t8990 = t3104 * t1795;
    let t8992 = t189 * t1636;
    (t8980, t8982, t8984, t8986, t8987, t8988, t8990, t8992)
}
