//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta788 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2601;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta788(t14472: f64, t1580: f64, t2439: f64, t2444: f64, t6049: f64, t689: f64, t136: f64, t2457: f64, t41011: f64, t6048: f64, t10504: f64, t6071: f64, t11007: f64, t252: f64, t2782: f64, t886: f64, t14481: f64, t1569: f64, t18805: f64, t41066: f64, t10995: f64, t122: f64, t18796: f64, t2466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61400, t61403, t61407, t61411) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2601(t14472, t1580, t2439, t2444, t6049, t689, t136, t2457, t41011, t6048, t10504, t6071);
        let (t61419, t61422, t61430, t61437) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2602(t11007, t252, t2782, t6048, t886, t14481, t1569, t18805, t41066, t10995, t122, t18796, t2466);
    (t61400, t61403, t61407, t61411, t61419, t61422, t61430, t61437)
}
