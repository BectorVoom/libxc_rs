//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 994/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk994<F: Float>(t10504: F, t10509: F, t10512: F, t10596: F, t10597: F, t10599: F, t10600: F, t10614: F, t10616: F, t10618: F, t10620: F, t10622: F, t10626: F, t10628: F, t10631: F, t7474: F, t7478: F) -> F {
    let t11206 = t10504 - t10509 + t10512 - t10596 - t10597 - t10599 - t10600 - t7474 - t7478 - t10614 + t10616 - t10618 - t10620 - t10622 - t10626 + t10628 + t10631;
    t11206
}
