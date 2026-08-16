//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1998/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1998<F: Float>(t1078: F, t1982: F, t93488: F, t25604: F, t25610: F, t3093: F, t4975: F, t3058: F, t8521: F, t3143: F, t7135: F, t11865: F, t25516: F) -> (F, F, F, F, F, F) {
    let t93490 = t1982 * t93488 * t1078;
    let t93497 = t25610 * t25604;
    let t93498 = t3093 * t4975;
    let t93502 = t3058 * t8521;
    let t93516 = t3143 * t7135;
    let t93543 = t11865 * t25516;
    (t93490, t93497, t93498, t93502, t93516, t93543)
}
