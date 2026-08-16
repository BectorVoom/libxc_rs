//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 857/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk857(t6711: f64, t8406: f64, t6717: f64, t204: f64, t1458: f64, t1580: f64, t1599: f64, t2476: f64, t2804: f64, t2807: f64, t557: f64, t574: f64, t597: f64, t6710: f64, t6716: f64, t6963: f64, t7012: f64, t7015: f64, t7019: f64, t7023: f64, t7027: f64, t7031: f64, t7037: f64, t7040: f64, t7049: f64, t8393: f64, t8398: f64, t8403: f64, t8407: f64, t8411: f64) -> f64 {
    let t8414 = t6711 * t8406;
    let t8417 = t6717 * t8406;
    let t8420 = t204 * t8406;
    let t8432 = -0.61348681526273199482e1_f64 * t574 * t8393 + 0.61348681526273199482e1_f64 * t1580 * t2804 + 0.61348681526273199482e1_f64 * t597 * t8398 - 0.47667319935800568892e0_f64 * t1599 * t2807 - 0.47667319935800568892e0_f64 * t557 * t8403 - 0.14300195980740170668e1_f64 * t6963 * t8407 + 0.21450293971110256002e1_f64 * t8411 * t1458 - 0.23005755572352449806e2_f64 * t6710 * t8414 + 0.13803453343411469884e2_f64 * t6716 * t8417 + 0.92023022289409799224e1_f64 * t2476 * t8420 - 0.76685851907841499352e0_f64 * t7012 + 0.76685851907841499352e0_f64 * t7015 + 0.17041300423964777634e0_f64 * t7019 - 0.17041300423964777634e0_f64 * t7023 + 0.59584149919750711116e-1_f64 * t7027 - 0.59584149919750711116e-1_f64 * t7031 - 0.38342925953920749676e0_f64 * t7037 + 0.38342925953920749676e0_f64 * t7040 + 0.19171462976960374838e0_f64 * t7049;
    t8432
}
