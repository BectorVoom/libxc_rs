//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1310/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1310<F: Float>(t11396: F, t13780: F, t13859: F, t3990: F, t11732: F, t3989: F, t3991: F, t15338: F, t4414: F, t11509: F, t3950: F, t833: F, t850: F) -> (F, F, F, F) {
    let t56757 = t13859 * t3990 * t13780 * t11396;
    let t56761 = t3989 * t3990 * t3991 * t11732;
    let t56769 = t4414 * t15338;
    let t56773 = t850 * t11509 * t3950 * t833;
    (t56757, t56761, t56769, t56773)
}
