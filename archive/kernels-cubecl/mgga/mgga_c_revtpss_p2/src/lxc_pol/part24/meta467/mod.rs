//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1442;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta467<F: Float>(t14472: F, t1580: F, t2439: F, t136: F, t2457: F, t41011: F, t6048: F, t10504: F, t6071: F, t18317: F, t2435: F, t10815: F, t6019: F, t10845: F, t18531: F, t18622: F, t6016: F, t853: F, t18432: F, t40336: F, t5977: F, t18441: F, t9775: F, t10716: F, t18402: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t61400, t61407, t61411, t61448, t61570) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1442::<F>(t14472, t1580, t2439, t136, t2457, t41011, t6048, t10504, t6071, t18317, t2435, t10815, t6019);
        let (t61572, t61576, t61579, t61623, t61625, t61645, t61675) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1443::<F>(t10845, t18531, t18622, t6016, t853, t18432, t40336, t5977, t18441, t9775, t10716, t18402);
    (t61400, t61407, t61411, t61448, t61570, t61572, t61576, t61579, t61623, t61625, t61645, t61675)
}
