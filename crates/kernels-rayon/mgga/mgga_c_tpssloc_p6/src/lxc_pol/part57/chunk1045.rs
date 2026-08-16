//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1045/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1045(t127560: f64, t127562: f64, t128908: f64, t128909: f64, t128922: f64, t128924: f64, t128926: f64, t128943: f64, t128970: f64, t1869: f64, t2075: f64, t27993: f64, t28002: f64, t28030: f64, t29197: f64, t29378: f64, t33133: f64, t33204: f64, t33350: f64, t4028: f64, t574: f64, t7451: f64, t7458: f64, t7890: f64, t7941: f64, t8450: f64, t8529: f64) -> f64 {
    let t128973 = t8450 * t29378 - t127560 - t127562 - t27993 * t2075 + 2.0_f64 * t33133 * t7941 - 2.0_f64 * t7451 * t7890 - t1869 * t29197 + t128908 + t128909 - 4.0_f64 * t28002 * t8529 - 4.0_f64 * t4028 * t33350 - 4.0_f64 * t4028 * t33204 - 2.0_f64 * t28030 * t8529 - 4.0_f64 * t7458 * t33350 - t128922 - t128924 + t128926 + (t128943 + t128970) * t574;
    t128973
}
