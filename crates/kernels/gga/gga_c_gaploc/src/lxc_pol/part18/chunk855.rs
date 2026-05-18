//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 855/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk855<F: Float>(t6711: F, t8406: F, t6717: F, t204: F, t1458: F, t1580: F, t1599: F, t2476: F, t2804: F, t2807: F, t557: F, t574: F, t597: F, t6710: F, t6716: F, t6963: F, t7012: F, t7015: F, t7019: F, t7023: F, t7027: F, t7031: F, t7037: F, t7040: F, t7049: F, t8393: F, t8398: F, t8403: F, t8407: F, t8411: F) -> F {
    let t8414 = t6711 * t8406;
    let t8417 = t6717 * t8406;
    let t8420 = t204 * t8406;
    let t8432 = -F::new(0.61348681526273199482e1) * t574 * t8393 + F::new(0.61348681526273199482e1) * t1580 * t2804 + F::new(0.61348681526273199482e1) * t597 * t8398 - F::new(0.47667319935800568892e0) * t1599 * t2807 - F::new(0.47667319935800568892e0) * t557 * t8403 - F::new(0.14300195980740170668e1) * t6963 * t8407 + F::new(0.21450293971110256002e1) * t8411 * t1458 - F::new(0.23005755572352449806e2) * t6710 * t8414 + F::new(0.13803453343411469884e2) * t6716 * t8417 + F::new(0.92023022289409799224e1) * t2476 * t8420 - F::new(0.76685851907841499352e0) * t7012 + F::new(0.76685851907841499352e0) * t7015 + F::new(0.17041300423964777634e0) * t7019 - F::new(0.17041300423964777634e0) * t7023 + F::new(0.59584149919750711116e-1) * t7027 - F::new(0.59584149919750711116e-1) * t7031 - F::new(0.38342925953920749676e0) * t7037 + F::new(0.38342925953920749676e0) * t7040 + F::new(0.19171462976960374838e0) * t7049;
    t8432
}
