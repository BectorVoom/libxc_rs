//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1297/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1297<F: Float>(t13917: F, t343: F, t53799: F, t54590: F, t824: F, t11396: F, t13780: F, t13859: F, t3990: F, t11732: F, t3989: F, t3991: F) -> (F, F, F) {
    let t56753 = t13917 * t53799 * t824 * t54590 * t343;
    let t56757 = t13859 * t3990 * t13780 * t11396;
    let t56761 = t3989 * t3990 * t3991 * t11732;
    (t56753, t56757, t56761)
}
