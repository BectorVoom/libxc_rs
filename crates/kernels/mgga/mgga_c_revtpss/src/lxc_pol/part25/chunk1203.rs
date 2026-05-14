//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1203/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1203<F: Float>(t25861: F, t4254: F, t25188: F, t7316: F, t10426: F, t196: F, t197: F, t2035: F, t28167: F, t8996: F, t9984: F, t26090: F, t7235: F, t2320: F, t569: F, t7221: F, t94369: F, t94371: F, t94374: F, t94376: F, t94940: F, t94942: F, t94944: F, t94994: F, t94998: F, t95001: F, t95005: F, t95008: F, t95011: F, t95013: F) -> (F,) {
    let t95015 = 12.0 * t4254 * t25861;
    let t95017 = 3.0 * t25188 * t7316;
    let t95019 = t10426 * t196 * t197;
    let t95020 = t95019 * t2035;
    let t95023 = 18.0 * t28167 * t8996 * t9984;
    let t95025 = 3.0 * t7235 * t26090;
    let t95026 = -3.0 * t2320 * t7221 + t569 * t94994 + t94369 - t94371 - t94374 + t94376 + t94940 - t94942 - t94944 - t94998 + t95001 + t95005 + t95008 - t95011 - t95013 - t95015 - t95017 + t95020 + t95023 + t95025;
    (t95026,)
}
