//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1855/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1855<F: Float>(t23040: F, t4166: F, t831: F, t4191: F, t81749: F, t4240: F, t13248: F, t25084: F, t13326: F, t23146: F, t13210: F, t13306: F) -> (F, F, F, F, F, F, F) {
    let t87261 = t4166 * t23040;
    let t87262 = t87261 * t831;
    let t87270 = t81749 * t4191;
    let t87272 = t81749 * t4240;
    let t87274 = t25084 * t13248;
    let t87276 = t23146 * t13326;
    let t87278 = t23146 * t13210;
    let t87280 = t23146 * t13306;
    (t87262, t87270, t87272, t87274, t87276, t87278, t87280)
}
