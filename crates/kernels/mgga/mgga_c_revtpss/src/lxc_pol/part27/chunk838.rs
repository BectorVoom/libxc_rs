//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 838/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk838<F: Float>(t2408: F, t2411: F, t262: F, t775: F, t10566: F, t10568: F, t10570: F, t10575: F, t10577: F, t10580: F, t10582: F, t10584: F, t2403: F, t2430: F, t4541: F, t9514: F, t9517: F, t9521: F) -> (F,) {
    let t11084 = t2408 * t2411;
    let t11088 = t262 * t775;
    let t11092 = -9.0 * t11084 * t2403 * t775 + 18.0 * t11088 * t2430 * t4541 + t10566 - t10568 + t10570 - t10575 + t10577 + t10580 + t10582 - t10584 + t9514 - t9517 - t9521;
    (t11092,)
}
