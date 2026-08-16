//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2173/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2173<F: Float>(t98959: F, t98981: F, t99008: F, t99037: F, t99059: F, t99079: F, t99098: F, t99116: F, t27316: F, t686: F, t72: F, t25375: F) -> (F, F, F) {
    let t99119 = t98959 + t98981 + t99008 + t99037 + t99059 + t99079 + t99098 + t99116;
    let t99125 = t27316 * t72 * t686;
    let t99127 = F::cast_from(0.28912093960683998208e-1_f64) * t25375 * t99125;
    (t99119, t99125, t99127)
}
