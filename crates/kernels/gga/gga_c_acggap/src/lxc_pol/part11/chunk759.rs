//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 759/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk759<F: Float>(t1967: F, t2299: F, t2294: F, t7840: F, t7845: F, t7847: F, t7850: F, t7854: F, t7863: F, t7864: F, t8963: F, t8967: F, t8971: F, t8973: F, t8975: F, t8979: F) -> (F,) {
    let t8981 = t1967 * t2299;
    let t8983 = t1967 * t2294;
    let t8989 = 0.31448092289604152068e-3 * t8963 - 0.47172138434406228102e-3 * t8967 + 0.15724046144802076034e-3 * t8971 + 0.32155513588552302729e-2 * t8973 - 0.28303283060643736861e-2 * t8975 - 0.21437009059034868486e-3 * t8979 - 0.47172138434406228102e-2 * t8981 + 0.12862205435420921092e-2 * t8983 + 0.15724046144802076034e-3 * t7840 + 0.10482697429868050689e-3 * t7845 - 0.10718504529517434243e-3 * t7847 + t7850 + t7854 + t7863 - 7.0 / 288.0 * t7864;
    (t8989,)
}
