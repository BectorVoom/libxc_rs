//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1217/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1217<F: Float>(t1949: F, t22: F, t25402: F, t93134: F, t1954: F, t39643: F, t7056: F, t25296: F, t25310: F, t25313: F, t686: F, t72: F) -> (F, F, F, F, F) {
    let t93136 = t25402 * t1949 * t22;
    let t93138 = F::new(0.43639970290213137151e-3) * t93134 * t93136;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93142 = F::new(0.51727911450665971904e-3) * t93140 * t93136;
    let t93143 = t25310 * t25296;
    let t93146 = t25313 * t72 * t686;
    (t93138, t93139, t93142, t93143, t93146)
}
