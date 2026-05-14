//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 990/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk990<F: Float>(t1428: F, t25620: F, t457: F, t1433: F, t25656: F, t12878: F, t12880: F, t1421: F, t19227: F, t19235: F, t19237: F, t19255: F, t19271: F, t19278: F, t19318: F, t19320: F, t26667: F, t26671: F, t26675: F, t26678: F, t26681: F, t26685: F, t26689: F, t26692: F, t456: F, t5913: F) -> (F, F, F) {
    let t26697 = t1428 * t25620;
    let t26698 = t457 * t26697;
    let t26703 = t1433 * t25656;
    let t26704 = t457 * t26703;
    let t26707 = -0.52563437333333333332e-2 * t5913 * t26667 + 0.98556445e-3 * t1421 * t26671 - 0.65704296666666666667e-3 * t1421 * t26675 - 0.13140859333333333333e-2 * t1421 * t26678 + 0.10950716111111111111e-2 * t1421 * t26681 - t19227 - 0.295669335e-2 * t1421 * t26685 - 0.295669335e-2 * t1421 * t26689 - 0.87605728888888888887e-3 * t26692 + 0.43802864444444444443e-3 * t19235 - 0.87605728888888888887e-3 * t19237 + t19255 + 0.17521145777777777778e-2 * t19271 + t19278 + 0.1478346675e-2 * t456 * t26698 - 0.32852148333333333333e-3 * t12878 + 0.21901432222222222222e-3 * t12880 - 0.98556445e-3 * t456 * t26704 + t19318 - t19320;
    (t26697, t26703, t26707)
}
