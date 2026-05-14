//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1034/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1034<F: Float>(t1284: F, t3555: F, t3624: F, t221: F, t462: F, t68: F, t461: F, t1209: F, t3766: F, t5330: F, t11772: F, t3623: F, t3717: F, t1263: F, t675: F, t1122: F, t247: F) -> (F, F, F, F, F, F, F) {
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    let t12851 = t221 * t68 * t462;
    let t12853 = 5.0 / 1296.0 * t461 * t12851;
    let t12854 = t1209 * t3766;
    let t12855 = t12854 * t5330;
    let t12865 = t3623 * t11772;
    let t12866 = t3717 * t12865;
    let t12879 = t675 * t1263;
    let t12881 = t247 * t12879 * t1122;
    (t12832, t12853, t12855, t12865, t12866, t12879, t12881)
}
