//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 861/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk861<F: Float>(t1730: F, t3564: F, t10603: F, t10607: F, t10611: F, t10614: F, t10616: F, t10618: F, t10620: F, t10622: F, t10626: F, t10628: F, t10631: F, t10633: F, t10634: F, t10657: F, t256: F, t267: F, t7474: F, t7478: F) -> (F, F) {
    let t10661 = 4.0 / 15.0 * t1730 * t3564;
    let t10662 = t10603 * t256 / 3.0 + t10607 / 3.0 + 0.60777777777777777777e-1 * t10611 - t7474 - t7478 - t10614 + t10616 - t10618 - t10620 - t10622 - t10626 + t10628 + t10631 + t10633 - 2.0 / 45.0 * t10634 - t10657 * t267 / 15.0 + t10661;
    (t10661, t10662)
}
