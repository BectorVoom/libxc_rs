//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1116/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1116<F: Float>(t12808: F, t5330: F, t3153: F, t3601: F, t1284: F, t3555: F, t3624: F, t221: F, t462: F, t68: F, t461: F, t1209: F, t3766: F) -> (F, F, F, F, F) {
    let t12809 = t12808 * t5330;
    let t12810 = t3601 * t3153;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    let t12851 = t221 * t68 * t462;
    let t12853 = F::new(5.0) / F::new(1296.0) * t461 * t12851;
    let t12854 = t1209 * t3766;
    (t12809, t12810, t12832, t12853, t12854)
}
