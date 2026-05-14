//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1014/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1014<F: Float>(t14581: F, t8244: F, t4223: F, t8275: F, t6336: F, t6382: F, t27068: F, t27071: F, t27074: F, t27076: F, t27079: F, t27082: F, t27084: F, t27087: F, t27090: F, t27093: F, t27096: F, t27099: F, t27102: F, t27105: F, t27107: F) -> (F, F, F, F) {
    let t27109 = t14581 * t8244;
    let t27111 = t4223 * t8275;
    let t27113 = t6382 * t6336;
    let t27115 = -t27068 / 12.0 - t27071 / 36.0 + 3.0 / 128.0 * t27074 + t27076 / 96.0 + t27079 / 4.0 - t27082 / 288.0 + t27084 / 96.0 + t27087 / 864.0 + t27090 / 576.0 - t27093 / 24.0 - t27096 / 64.0 + t27099 / 4.0 + t27102 / 192.0 - t27105 / 72.0 - t27107 / 12.0 - t27109 / 12.0 + t27111 / 256.0 - t27113 / 24.0;
    (t27109, t27111, t27113, t27115)
}
