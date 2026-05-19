//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 871/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk871<F: Float>(t72: F, t7531: F, t686: F, t7284: F, t7289: F, t136: F, t2102: F, t2457: F, t25944: F, t25950: F, t7515: F, t213: F, t7506: F) -> (F, F, F, F, F, F) {
    let t26270 = t7531 * t72;
    let t26271 = t26270 * t686;
    let t26272 = t7284 * t26271;
    let t26274 = t7289 * t26271;
    let t26276 = t2102 * t136;
    let t26277 = t26276 * t2457;
    let t26279 = F::cast_from(0.17135234354032049604e-2_f64) * t25944 * t26277;
    let t26280 = t25950 * t7515;
    let t26282 = t213 * t7506;
    (t26272, t26274, t26277, t26279, t26280, t26282)
}
