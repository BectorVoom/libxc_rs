//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1117/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1117<F: Float>(t18615: F, t231: F, t827: F, t828: F, t221: F, t2485: F, t6017: F, t2484: F, t125: F, t5962: F, t2747: F, t837: F, t2723: F, t4423: F, t4364: F, t4365: F) -> (F, F, F, F, F, F) {
    let t18616 = t18615 * t231;
    let t18618 = t827 * t828 * t18616;
    let t18622 = t2485 * t221 * t6017;
    let t18623 = t2484 * t18622;
    let t18627 = t125 * t5962;
    let t18629 = t2747 * t18627 * t837;
    let t18632 = t2723 * t4423;
    let t18634 = t4364 * t4365 * t18632;
    (t18616, t18618, t18623, t18629, t18632, t18634)
}
