//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1930/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1930<F: Float>(t1579: F, t231: F, t2645: F, t14939: F, t1955: F, t99270: F, t1559: F, t2828: F, t2722: F, t4533: F, t836: F, t2723: F) -> (F, F, F, F, F, F, F) {
    let t99289 = t1579 * t2645 * t231;
    let t99300 = t1955 * t14939;
    let t99303 = t1955 * t99270;
    let t99309 = t1559 * t2828;
    let t99315 = t1579 * t2722;
    let t99316 = t99315 * t231;
    let t99360 = t4533 * t836 * t231;
    let t99369 = t99315 * t2723;
    (t99289, t99300, t99303, t99309, t99316, t99360, t99369)
}
