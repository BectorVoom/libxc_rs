//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2506/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2506<F: Float>(t14869: F, t9775: F, t10899: F, t136: F, t216: F, t14786: F, t231: F, t40834: F, t854: F, t14833: F, t236: F, t2453: F, t9794: F) -> (F, F, F, F, F) {
    let t50443 = t9775 * t14869;
    let t50446 = t216 * t10899 * t136;
    let t50451 = t14786 * t231;
    let t50453 = t40834 * t854 * t50451;
    let t50454 = F::cast_from(0.30492001685571196935e-4_f64) * t50453;
    let t50457 = t2453 * t236 * t9794 * t14833;
    (t50443, t50446, t50451, t50454, t50457)
}
