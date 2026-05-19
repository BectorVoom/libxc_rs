//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 854/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk854<F: Float>(t1392: F, t2787: F, t1391: F, t600: F, t7861: F, t568: F, t1508: F, t999: F, t2822: F, t524: F, t1000: F, t1013: F, t1456: F, t1520: F, t1580: F, t1617: F, t1641: F, t193: F, t2487: F, t2810: F, t2816: F, t2865: F, t2898: F, t4372: F, t4540: F, t4637: F, t574: F, t587: F, t597: F, t6744: F, t6982: F, t6987: F, t8347: F, t8352: F, t8355: F, t8358: F, t8361: F, t8367: F) -> F {
    let t8370 = t1392 * t2787;
    let t8371 = t1391 * t8370;
    let t8380 = t600 * t7861;
    let t8381 = t568 * t8380;
    let t8384 = t1508 * t999;
    let t8387 = t524 * t2822;
    let t8392 = F::cast_from(0.1022478025437886658e1_f64) * t6982 - F::cast_from(0.1022478025437886658e1_f64) * t6987 + F::cast_from(0.47667319935800568892e0_f64) * t1456 * t8347 + F::cast_from(0.46011511144704899612e1_f64) * t1617 * t2865 + F::cast_from(0.42900587942220512003e1_f64) * t8352 * t6744 - F::cast_from(0.21450293971110256001e1_f64) * t4540 * t8355 + F::cast_from(0.30674340763136599741e2_f64) * t597 * t8358 - F::cast_from(0.12269736305254639896e2_f64) * t574 * t8361 + F::cast_from(0.92686455430723328401e-1_f64) * t2898 * t4372 - F::cast_from(0.11360866949309851756e0_f64) * t587 * t8367 + F::cast_from(0.11360866949309851756e0_f64) * t2487 * t8371 + F::cast_from(0.23005755572352449806e1_f64) * t4637 * t1013 - F::cast_from(0.79445533226334281487e-1_f64) * t1000 * t1520 + F::cast_from(0.46011511144704899612e1_f64) * t1580 * t2816 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t8381 + F::cast_from(0.35750489951850426669e0_f64) * t8384 * t193 + F::cast_from(0.71500979903700853338e0_f64) * t8387 * t193 - F::cast_from(0.61348681526273199482e1_f64) * t1641 * t2810;
    t8392
}
