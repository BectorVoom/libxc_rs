//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 800/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk800<F: Float>(t1508: F, t999: F, t2822: F, t524: F, t1000: F, t1013: F, t1456: F, t1520: F, t1580: F, t1617: F, t1641: F, t193: F, t2487: F, t2810: F, t2816: F, t2865: F, t2898: F, t4372: F, t4540: F, t4637: F, t574: F, t587: F, t597: F, t6744: F, t6982: F, t6987: F, t8347: F, t8352: F, t8355: F, t8358: F, t8361: F, t8367: F, t8371: F, t8381: F) -> (F,) {
    let t8384 = t1508 * t999;
    let t8387 = t524 * t2822;
    let t8392 = 0.1022478025437886658e1 * t6982 - 0.1022478025437886658e1 * t6987 + 0.47667319935800568892e0 * t1456 * t8347 + 0.46011511144704899612e1 * t1617 * t2865 + 0.42900587942220512003e1 * t8352 * t6744 - 0.21450293971110256001e1 * t4540 * t8355 + 0.30674340763136599741e2 * t597 * t8358 - 0.12269736305254639896e2 * t574 * t8361 + 0.92686455430723328401e-1 * t2898 * t4372 - 0.11360866949309851756e0 * t587 * t8367 + 0.11360866949309851756e0 * t2487 * t8371 + 0.23005755572352449806e1 * t4637 * t1013 - 0.79445533226334281487e-1 * t1000 * t1520 + 0.46011511144704899612e1 * t1580 * t2816 + 0.23005755572352449806e1 * t597 * t8381 + 0.35750489951850426669e0 * t8384 * t193 + 0.71500979903700853338e0 * t8387 * t193 - 0.61348681526273199482e1 * t1641 * t2810;
    (t8392,)
}
