//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1442;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1443;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta467(t14472: f64, t1580: f64, t2439: f64, t136: f64, t2457: f64, t41011: f64, t6048: f64, t10504: f64, t6071: f64, t18317: f64, t2435: f64, t10815: f64, t6019: f64, t10845: f64, t18531: f64, t18622: f64, t6016: f64, t853: f64, t18432: f64, t40336: f64, t5977: f64, t18441: f64, t9775: f64, t10716: f64, t18402: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61400, t61407, t61411, t61448, t61570) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1442(t14472, t1580, t2439, t136, t2457, t41011, t6048, t10504, t6071, t18317, t2435, t10815, t6019);
        let (t61572, t61576, t61579, t61623, t61625, t61645, t61675) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1443(t10845, t18531, t18622, t6016, t853, t18432, t40336, t5977, t18441, t9775, t10716, t18402);
    (t61400, t61407, t61411, t61448, t61570, t61572, t61576, t61579, t61623, t61625, t61645, t61675)
}
