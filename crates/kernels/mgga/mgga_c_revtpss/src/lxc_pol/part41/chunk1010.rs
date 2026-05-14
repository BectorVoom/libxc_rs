//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1010/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1010<F: Float>(t3555: F, t3754: F, t1248: F, t3153: F, t3566: F, t1269: F, t1284: F, t1209: F, t1204: F, t3781: F, t5462: F, t5477: F, t3634: F, t828: F, t3624: F, t3746: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12709 = t3555 * t3754;
    let t12712 = t1248 * t3153;
    let t12717 = t3566 * t3754;
    let t12722 = t1284 * t1269;
    let t12723 = t1209 * t12722;
    let t12744 = t1204 * t3781;
    let t12751 = t1209 * t5462;
    let t12756 = t1209 * t5477;
    let t12772 = t828 * t3634;
    let t12784 = t3746 * t3624;
    (t12709, t12712, t12717, t12723, t12744, t12751, t12756, t12772, t12784)
}
