//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 763/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk763(t3128: f64, t5626: f64, t3133: f64, t1027: f64, t1790: f64, t1991: f64, t1855: f64, t3053: f64, t3057: f64, t3065: f64, t3121: f64, t1971: f64, t3707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8972 = t3128 * t5626;
    let t8974 = t3133 * t5626;
    let t8976 = t1027 * t1790;
    let t8978 = t1027 * t1991;
    let t8980 = t3053 * t1855;
    let t8982 = t3057 * t1855;
    let t8984 = t3121 * t3065;
    let t8986 = t1971 * t3707;
    (t8972, t8974, t8976, t8978, t8980, t8982, t8984, t8986)
}
