//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 575/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk575<F: Float>(t1684: F, t1741: F, t1788: F, t1028: F, t395: F, t1691: F, t2679: F, t11: F, t2673: F, t625: F, t1365: F, t4: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2692 = 4.0 / 45.0 * t1684;
    let t2693 = 4.0 / 45.0 * t1741;
    let t2694 = 4.0 / 45.0 * t1788;
    let t2696 = t395 * t1028;
    let t2698 = t1691 * t2679;
    let t2699 = t11 * t2698;
    let t2701 = t625 * t2673;
    let t2702 = t11 * t2701;
    let t2704 = t4 * t1365;
    (t2692, t2693, t2694, t2696, t2698, t2699, t2701, t2702, t2704)
}
