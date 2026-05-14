//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 678/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk678<F: Float>(t1417: F, t4660: F, t11325: F, t11330: F, t11335: F, t11338: F, t11340: F, t11342: F, t11344: F, t11347: F, t11350: F, t11387: F, t1421: F, t456: F, t5101: F, t707: F) -> (F, F) {
    let t11390 = t1417 * t4660;
    let t11392 = 0.39422577999999999999e-2 * t1421 * t11325 - 0.65704296666666666666e-2 * t1421 * t11330 + 0.22175200125e-2 * t1421 * t11335 - 0.19711289e-2 * t11338 + 0.1478346675e-2 * t11340 + 0.295669335e-2 * t11342 + 0.65704296666666666665e-3 * t11344 + 0.1478346675e-2 * t456 * t11347 - 0.98556445e-3 * t11350 - 0.98556445e-3 * t456 * t11387 + 0.39422577999999999999e-2 * t11390;
    let t11393 = t707 * t5101;
    (t11392, t11393)
}
