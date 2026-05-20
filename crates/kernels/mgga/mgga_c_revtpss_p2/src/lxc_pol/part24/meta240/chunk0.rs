//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1001/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1001<F: Float>(t14219: F, t2457: F, t10139: F, t1892: F, t4086: F, t786: F, t2470: F, t5740: F, t4101: F, t1432: F, t5763: F, t3920: F, t5603: F) -> (F, F, F, F, F, F, F, F) {
    let t14220 = t14219 * t2457;
    let t14221 = t10139 * t14220;
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    let t14242 = t5740 * t2470;
    let t14243 = t4101 * t14242;
    let t14252 = t1432 * t5763 * t2470;
    let t14280 = t5603 * t3920;
    (t14220, t14221, t14238, t14239, t14242, t14243, t14252, t14280)
}
