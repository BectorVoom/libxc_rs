//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1031/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1031<F: Float>(t1444: F, t32211: F, t5673: F, t32206: F, t4075: F, t8705: F, t1419: F, t8477: F, t1385: F, t9656: F) -> (F, F, F, F, F) {
    let t32213 = t5673 * t32211 * t1444;
    let t32214 = t32206 * t32213;
    let t32237 = t8705 * t4075;
    let t32247 = t8477 * t1419;
    let t32250 = t9656 * t1385;
    (t32213, t32214, t32237, t32247, t32250)
}
