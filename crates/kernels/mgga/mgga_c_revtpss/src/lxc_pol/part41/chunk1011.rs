//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1011/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1011<F: Float>(t3618: F, t828: F, t1209: F, t3781: F, t5330: F, t1284: F, t3555: F, t3624: F, t1121: F, t3603: F, t606: F, t221: F, t462: F, t68: F, t461: F, t3766: F) -> (F, F, F, F, F, F) {
    let t12787 = t828 * t3618;
    let t12808 = t1209 * t3781;
    let t12809 = t12808 * t5330;
    let t12831 = t3555 * t1284;
    let t12832 = t12831 * t3624;
    let t12839 = t3603 * t1121;
    let t12840 = t12839 * t606;
    let t12851 = t221 * t68 * t462;
    let t12853 = 5.0 / 1296.0 * t461 * t12851;
    let t12854 = t1209 * t3766;
    (t12787, t12809, t12832, t12840, t12853, t12854)
}
