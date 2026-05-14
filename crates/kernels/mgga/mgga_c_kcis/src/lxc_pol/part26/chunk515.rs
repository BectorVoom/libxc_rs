//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 515/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk515<F: Float>(t1102: F, t1360: F, t1924: F, t344: F, t486: F, t5454: F, t5460: F, t5465: F, t5483: F, t5486: F, t5490: F, t5495: F, t5500: F, t5528: F, t5623: F, t5451: F) -> (F,) {
    let t5626 = 0.98556445e-3 * t1102 * t5454 + 0.7391733375e-3 * t1102 * t5460 - 0.1478346675e-2 * t1102 * t5465 + 0.1478346675e-2 * t344 * t5483 - 0.65704296666666666667e-3 * t5486 - 0.65704296666666666667e-3 * t1102 * t5490 - 0.1478346675e-2 * t1102 * t5495 + 0.19711289e-2 * t1102 * t5500 - 0.98556445e-3 * t344 * t5528 - 4.0 * t1360 * t1924 - 4.0 * t486 * t5623;
    let t5627 = t5451 + t5626;
    (t5627,)
}
