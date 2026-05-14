//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 854/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk854<F: Float>(t10535: F, t5089: F, t11: F, t10524: F, t1691: F, t10539: F, t2704: F, t10353: F, t625: F, t10357: F, t10555: F, t10550: F, t3466: F, t395: F, t3470: F, t3474: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10560 = t5089 * t10535;
    let t10561 = t11 * t10560;
    let t10563 = t1691 * t10524;
    let t10564 = t11 * t10563;
    let t10566 = t1691 * t10539;
    let t10567 = t2704 * t10566;
    let t10569 = t625 * t10353;
    let t10570 = t11 * t10569;
    let t10572 = t625 * t10357;
    let t10573 = t2704 * t10572;
    let t10575 = t1691 * t10555;
    let t10576 = t11 * t10575;
    let t10578 = t625 * t10550;
    let t10579 = t11 * t10578;
    let t10581 = t395 * t3466;
    let t10583 = t395 * t3470;
    let t10585 = t395 * t3474;
    (t10561, t10564, t10567, t10570, t10573, t10576, t10579, t10581, t10583, t10585)
}
