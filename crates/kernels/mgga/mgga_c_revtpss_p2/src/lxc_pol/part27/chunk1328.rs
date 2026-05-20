//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1328/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1328<F: Float>(t10259: F, t2163: F, t2331: F, t2372: F, t27060: F, t29432: F, t651: F, t671: F, t94361: F, t94369: F, t94371: F, t94374: F, t94376: F, t94940: F, t94942: F, t94944: F, t94998: F, t95001: F, t95005: F, t95008: F, t95011: F, t95013: F, t95015: F, t96706: F) -> F {
    let t97537 = -F::new(2.0) * t10259 * t2163 * t651 - F::new(12.0) * t2331 * t27060 - F::new(6.0) * t2372 * t27060 - F::new(6.0) * t2372 * t29432 - F::new(6.0) * t671 * t96706 + t94361 + t94369 - t94371 - t94374 + t94376 + t94940 - t94942 - t94944 - t94998 + t95001 + t95005 + t95008 - t95011 - t95013 - t95015;
    t97537
}
