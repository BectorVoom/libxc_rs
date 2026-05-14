//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 935/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk935<F: Float>(t2290: F, t7610: F, t30541: F, t30544: F, t30559: F, t30561: F, t30565: F, t30569: F, t30577: F, t34413: F, t34414: F, t34417: F, t34422: F, t34424: F, t34427: F, t34430: F, t34431: F, t34432: F, t34433: F) -> (F,) {
    let t34435 = t7610 * t2290;
    let t34437 = t34413 - t34414 + 0.80031500487063509016e-2 * t30541 - 0.12862205435420921092e-1 * t30544 - t34417 + 0.83861579438944405513e-3 * t30559 + 0.20965394859736101378e-2 * t30561 + 0.28582678745379824648e-3 * t30565 - t34422 - t34424 / 32.0 - t34427 / 64.0 - t30569 - t34430 - t34431 + t30577 + t34432 - 77.0 / 1728.0 * t34433 + 0.47172138434406228102e-3 * t34435;
    (t34437,)
}
