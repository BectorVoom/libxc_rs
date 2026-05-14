//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1054/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1054<F: Float>(t30212: F, t32377: F, t32379: F, t32380: F, t32384: F, t32385: F, t32386: F, t32387: F, t33936: F, t36870: F, t38859: F, t38863: F, t38867: F, t38871: F, t38875: F, t38879: F, t38886: F) -> (F,) {
    let t41382 = 0.18868855373762491241e-2 * t38859 + 0.94344276868812456204e-2 * t38863 - 0.62896184579208304136e-2 * t38867 - 0.37737710747524982482e-2 * t38871 + 0.20965394859736101379e-3 * t38875 - 0.47172138434406228104e-2 * t38879 - t32377 - t32379 + t32380 + t32384 - t32385 - t32386 - t32387 - 0.12579236915841660828e-2 * t30212 - t33936 + 0.10718504529517434243e-2 * t38886 - t36870;
    (t41382,)
}
