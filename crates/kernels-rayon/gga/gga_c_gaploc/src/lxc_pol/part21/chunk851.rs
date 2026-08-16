//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 851/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk851(t1392: f64, t2787: f64, t1391: f64, t600: f64, t7861: f64, t568: f64, t1508: f64, t999: f64, t2822: f64, t524: f64, t1000: f64, t1013: f64, t1456: f64, t1520: f64, t1580: f64, t1617: f64, t1641: f64, t193: f64, t2487: f64, t2810: f64, t2816: f64, t2865: f64, t2898: f64, t4372: f64, t4540: f64, t4637: f64, t574: f64, t587: f64, t597: f64, t6744: f64, t6982: f64, t6987: f64, t8347: f64, t8352: f64, t8355: f64, t8358: f64, t8361: f64, t8367: f64) -> f64 {
    let t8370 = t1392 * t2787;
    let t8371 = t1391 * t8370;
    let t8380 = t600 * t7861;
    let t8381 = t568 * t8380;
    let t8384 = t1508 * t999;
    let t8387 = t524 * t2822;
    let t8392 = 0.1022478025437886658e1_f64 * t6982 - 0.1022478025437886658e1_f64 * t6987 + 0.47667319935800568892e0_f64 * t1456 * t8347 + 0.46011511144704899612e1_f64 * t1617 * t2865 + 0.42900587942220512003e1_f64 * t8352 * t6744 - 0.21450293971110256001e1_f64 * t4540 * t8355 + 0.30674340763136599741e2_f64 * t597 * t8358 - 0.12269736305254639896e2_f64 * t574 * t8361 + 0.92686455430723328401e-1_f64 * t2898 * t4372 - 0.11360866949309851756e0_f64 * t587 * t8367 + 0.11360866949309851756e0_f64 * t2487 * t8371 + 0.23005755572352449806e1_f64 * t4637 * t1013 - 0.79445533226334281487e-1_f64 * t1000 * t1520 + 0.46011511144704899612e1_f64 * t1580 * t2816 + 0.23005755572352449806e1_f64 * t597 * t8381 + 0.35750489951850426669e0_f64 * t8384 * t193 + 0.71500979903700853338e0_f64 * t8387 * t193 - 0.61348681526273199482e1_f64 * t1641 * t2810;
    t8392
}
