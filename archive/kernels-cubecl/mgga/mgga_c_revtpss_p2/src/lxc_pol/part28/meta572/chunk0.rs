//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2034/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2034<F: Float>(t3058: F, t8521: F, t7135: F, t989: F, t25625: F, t7166: F, t11213: F, t1976: F, t11711: F, t25517: F, t11865: F, t25516: F) -> (F, F, F, F, F, F) {
    let t93502 = t3058 * t8521;
    let t93509 = t989 * t7135;
    let t93521 = t25625 * t7166;
    let t93528 = t11213 * t1976;
    let t93541 = t25517 * t11711;
    let t93543 = t11865 * t25516;
    (t93502, t93509, t93521, t93528, t93541, t93543)
}
