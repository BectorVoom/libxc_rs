//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 715/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk715(t1456: f64, t1562: f64, t1580: f64, t1641: f64, t2379: f64, t2382: f64, t2386: f64, t2392: f64, t2402: f64, t2418: f64, t4667: f64, t4730: f64, t4753: f64, t4950: f64, t4953: f64, t567: f64, t574: f64, t597: f64, t6637: f64, t6642: f64, t6649: f64, t6652: f64, t6656: f64, t6659: f64, t6665: f64, t6669: f64, t6673: f64, t6676: f64, t6679: f64, t6682: f64, t6689: f64) -> f64 {
    let t6692 = 0.61348681526273199482e1_f64 * t567 * t6637 - 0.47667319935800568892e0_f64 * t2402 * t4753 - 0.21450293971110256002e1_f64 * t6642 * t2386 + 0.14300195980740170668e1_f64 * t4950 * t2392 - 0.13803453343411469884e2_f64 * t1562 * t6649 - 0.21450293971110256002e1_f64 * t6652 * t2386 - 0.92023022289409799224e1_f64 * t574 * t6656 + 0.23005755572352449806e2_f64 * t597 * t6659 - 0.13803453343411469884e2_f64 * t4953 * t2418 - 0.92023022289409799224e1_f64 * t574 * t6665 + 0.43710935587469654631e2_f64 * t597 * t6669 + 0.46011511144704899612e1_f64 * t4730 * t6673 + 0.71500979903700853338e0_f64 * t6676 * t4667 + 0.30674340763136599741e2_f64 * t597 * t6679 - 0.18404604457881959845e2_f64 * t1562 * t6682 - 0.92023022289409799224e1_f64 * t1641 * t2379 + 0.23005755572352449806e2_f64 * t1580 * t2382 + 0.35750489951850426669e0_f64 * t1456 * t6689;
    t6692
}
