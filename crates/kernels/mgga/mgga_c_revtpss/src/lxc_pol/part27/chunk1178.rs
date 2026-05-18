//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1178/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1178<F: Float>(t3670: F, t7623: F, t2133: F, t816: F, t1224: F, t65: F, t3698: F, t26865: F, t4890: F, t3767: F, t3782: F, t1203: F, t5457: F) -> (F, F, F, F, F, F, F, F) {
    let t29040 = t3670 * t7623;
    let t29047 = t2133 * t816;
    let t29048 = t65 * t1224;
    let t29054 = t65 * t3698;
    let t29096 = t26865 * t4890;
    let t29097 = t3767 * t29096;
    let t29100 = t3782 * t29096;
    let t29159 = t5457 * t1203;
    (t29040, t29047, t29048, t29054, t29096, t29097, t29100, t29159)
}
