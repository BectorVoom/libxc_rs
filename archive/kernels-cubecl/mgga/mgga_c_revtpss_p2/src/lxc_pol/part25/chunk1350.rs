//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1350/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1350<F: Float>(t26090: F, t7235: F, t2320: F, t569: F, t7221: F, t94369: F, t94371: F, t94374: F, t94376: F, t94940: F, t94942: F, t94944: F, t94994: F, t94998: F, t95001: F, t95005: F, t95008: F, t95011: F, t95013: F, t95015: F, t95017: F, t95020: F, t95023: F) -> F {
    let t95025 = F::cast_from(3.0_f64) * t7235 * t26090;
    let t95026 = -F::cast_from(3.0_f64) * t2320 * t7221 + t569 * t94994 + t94369 - t94371 - t94374 + t94376 + t94940 - t94942 - t94944 - t94998 + t95001 + t95005 + t95008 - t95011 - t95013 - t95015 - t95017 + t95020 + t95023 + t95025;
    t95026
}
