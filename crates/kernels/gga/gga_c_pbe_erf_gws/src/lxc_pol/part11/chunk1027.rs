//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1027/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1027<F: Float>(t22669: F, t22674: F, t22676: F, t22679: F, t18899: F, t18961: F, t18968: F, t18970: F, t18973: F, t18977: F, t48510: F, t48511: F, t48512: F, t48439: F, t48476: F, t48487: F, t48490: F, t48494: F, t48500: F, t48509: F) -> (F, F, F, F, F) {
    let t48513 = 0.13012297059337829057e0 * t22669;
    let t48514 = 0.4155781415850207192e3 * t22674;
    let t48515 = 0.2077890707925103596e3 * t22676;
    let t48516 = 480.0 * t22679;
    let t48517 = -t18961 - t18968 + t18970 + t18973 - t18977 + t48510 - t48511 + t48512 - t48513 + t48514 - t48515 - t48516 - t18899;
    let t48520 = t48439 + t48476 + t48487 + t48490 + t48494 + t48500 + t48509 + t48517;
    (t48513, t48514, t48515, t48516, t48520)
}
