//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1351/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1351(t35119: f64, t6907: f64, t9263: f64, t993: f64, t2890: f64, t9267: f64, t10470: f64, t4418: f64, t10474: f64, t4425: f64, t20019: f64, t26984: f64, t6520: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35120 = 0.42603251059911944084e0_f64 * t35119;
    let t35122 = t9263 * t993 * t6907;
    let t35123 = 0.76685851907841499352e0_f64 * t35122;
    let t35125 = t9267 * t2890 * t6907;
    let t35126 = 0.36425779656224712192e1_f64 * t35125;
    let t35128 = 0.2556195063594716645e1_f64 * t4418 * t10470;
    let t35130 = 0.1022478025437886658e1_f64 * t4425 * t10474;
    let t35133 = 0.23833659967900284446e0_f64 * t26984 * t20019 * t6520;
    (t35120, t35123, t35126, t35128, t35130, t35133)
}
