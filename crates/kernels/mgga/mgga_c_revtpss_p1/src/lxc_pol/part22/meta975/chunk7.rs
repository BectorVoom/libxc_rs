//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3283/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3283<F: Float>(t39989: F, t61249: F, t61250: F, t61261: F, t61265: F, t61269: F, t61274: F, t61283: F, t61286: F, t61287: F, t61288: F, t61290: F, t61292: F, t61293: F, t61295: F, t61297: F, t61300: F, t61302: F, t61306: F) -> F {
    let t62267 = t61249 + t61250 + t61261 + t61265 + t61269 + t61274 + t61283 + t61286 - t39989 + t61287 + t61288 + t61290 - t61292 - t61293 - t61295 - t61297 + t61300 + t61302 + t61306;
    t62267
}
