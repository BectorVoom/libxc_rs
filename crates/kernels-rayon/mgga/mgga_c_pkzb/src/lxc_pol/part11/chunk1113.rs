//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1113/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1113(t1281: f64, t204: f64, t3026: f64, t1180: f64, t218: f64, t5555: f64, t1878: f64, t3061: f64, t3065: f64, t1184: f64, t6142: f64, t1220: f64, t6377: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22233 = t204 * t1281 * t3026;
    let t22234 = 0.12077e1_f64 * t22233;
    let t22290 = t218 * t5555 * t1180;
    let t22293 = t218 * t1878 * t3061;
    let t22294 = 0.82785e0_f64 * t22293;
    let t22296 = t218 * t1878 * t3065;
    let t22297 = 0.82785e0_f64 * t22296;
    let t22302 = 4.0_f64 / 3.0_f64 * t22233;
    let t22336 = 0.11958666666666666667e1_f64 * t22233;
    let t22391 = t6142 * t1184;
    let t22461 = t1220 * t6377;
    (t22233, t22234, t22290, t22293, t22294, t22296, t22297, t22302, t22336, t22391, t22461)
}
