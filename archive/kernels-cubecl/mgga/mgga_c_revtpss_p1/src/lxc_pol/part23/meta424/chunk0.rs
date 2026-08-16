//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1812/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1812<F: Float>(t18616: F, t827: F, t828: F, t221: F, t2485: F, t6017: F, t2484: F, t125: F, t5962: F, t2747: F, t837: F, t2723: F, t4423: F) -> (F, F, F, F, F, F) {
    let t18618 = t827 * t828 * t18616;
    let t18622 = t2485 * t221 * t6017;
    let t18623 = t2484 * t18622;
    let t18627 = t125 * t5962;
    let t18629 = t2747 * t18627 * t837;
    let t18632 = t2723 * t4423;
    (t18618, t18622, t18623, t18627, t18629, t18632)
}
