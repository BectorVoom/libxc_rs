//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 822/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk822<F: Float>(t1102: F, t344: F, t3743: F, t486: F, t5423: F, t5449: F, t5486: F, t7028: F, t7214: F, t7218: F, t7222: F, t7226: F, t7230: F, t7234: F, t7238: F, t7242: F, t7246: F, t7250: F, t7253: F) -> (F,) {
    let t7257 = -t3743 + 0.8760572888888888889e-3 * t5423 + 0.19711289e-2 * t5449 - 0.13140859333333333333e-2 * t5486 + 0.10950716111111111111e-2 * t1102 * t7214 + 0.19711289e-2 * t1102 * t7218 - 0.13140859333333333333e-2 * t1102 * t7222 - 0.13140859333333333333e-2 * t1102 * t7226 + 0.65704296666666666667e-3 * t1102 * t7230 + 0.7391733375e-3 * t344 * t7234 - 0.295669335e-2 * t1102 * t7238 + 0.1478346675e-2 * t344 * t7242 + 0.19711289e-2 * t344 * t7246 - 0.98556445e-3 * t344 * t7250 - 4.0 * t7253 - 4.0 * t486 * t7028;
    (t7257,)
}
