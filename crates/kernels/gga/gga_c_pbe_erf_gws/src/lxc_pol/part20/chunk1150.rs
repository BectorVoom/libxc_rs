//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1150/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1150<F: Float>(t11732: F, t3989: F, t3990: F, t3991: F, t15338: F, t4414: F, t11509: F, t3950: F, t833: F, t850: F, t14627: F, t15139: F, t2408: F, t2409: F, t26604: F, t51572: F, t53704: F, t53726: F, t53728: F, t56740: F, t56743: F, t56745: F, t56747: F, t56753: F, t56757: F, t8589: F) -> (F,) {
    let t56761 = t3989 * t3990 * t3991 * t11732;
    let t56769 = t4414 * t15338;
    let t56773 = t850 * t11509 * t3950 * t833;
    let t56775 = -t56740 / 96.0 - t56743 / 96.0 + 7.0 / 288.0 * t56745 - 7.0 / 2304.0 * t56747 - 35.0 / 432.0 * t51572 - t53704 + t56753 / 768.0 + t56757 / 768.0 - t56761 / 3072.0 + t2408 * t2409 * t8589 * t14627 / 24.0 + t26604 * t15139 / 96.0 + 7.0 / 36.0 * t56769 + t56773 / 96.0 - t53726 + t53728;
    (t56775,)
}
