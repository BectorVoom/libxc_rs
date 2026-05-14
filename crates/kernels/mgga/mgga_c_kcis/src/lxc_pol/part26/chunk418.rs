//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 418/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk418<F: Float>(t2751: F, t888: F, t221: F, t2423: F, t2427: F, t2430: F, t2482: F, t2486: F, t2494: F, t2529: F, t2718: F, t2720: F, t2725: F, t2729: F, t874: F, t889: F) -> (F, F) {
    let t2752 = t2751 * t888;
    let t2764 = t2718 * t221 - 0.13345e0 * t2720 * t889 + 0.890445125e-2 * t2725 * t2729 - 0.66725e-1 * t874 * t2752 + 0.66725e-1 * t874 * t2729 + 0.21667074074074074073e-1 * t2423 - 0.18571777777777777777e-1 * t2427 + 0.18571777777777777777e-1 * t2430 + 0.69644166666666666665e-2 * t2482 - 0.13928833333333333333e-1 * t2486 + 0.13928833333333333333e-1 * t2494 - 0.69644166666666666665e-2 * t2529;
    (t2752, t2764)
}
