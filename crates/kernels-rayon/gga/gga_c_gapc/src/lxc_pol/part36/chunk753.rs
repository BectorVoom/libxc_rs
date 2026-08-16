//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 753/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk753(t1045: f64, t5510: f64, t1043: f64, t1432: f64, t2982: f64, t2980: f64, t3128: f64, t5626: f64, t3133: f64, t1027: f64, t1790: f64, t1991: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8965 = t1045 * t5510;
    let t8966 = t1043 * t8965;
    let t8968 = t2982 * t1432;
    let t8969 = t2980 * t8968;
    let t8972 = t3128 * t5626;
    let t8974 = t3133 * t5626;
    let t8976 = t1027 * t1790;
    let t8978 = t1027 * t1991;
    (t8965, t8966, t8969, t8972, t8974, t8976, t8978)
}
