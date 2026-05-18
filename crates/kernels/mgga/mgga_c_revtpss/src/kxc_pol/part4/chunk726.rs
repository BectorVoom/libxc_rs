//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 726/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk726<F: Float>(t3682: F, t461: F, t1226: F, t140: F, t1222: F, t1225: F, t2258: F, t1012: F, t1224: F, t3367: F, t2251: F, t1121: F, t404: F) -> (F, F, F, F, F, F, F, F) {
    let t3684 = t461 * t3682 / F::new(432.0);
    let t3685 = t140 * t1226;
    let t3686 = t1222 * t3685;
    let t3688 = t1225 * t2258;
    let t3689 = t1012 * t3688;
    let t3692 = t1224 * t3367;
    let t3693 = t3692 * t2251;
    let t3694 = t1012 * t3693;
    let t3698 = F::new(1.0) / t404 / t1121;
    (t3684, t3685, t3686, t3688, t3689, t3693, t3694, t3698)
}
