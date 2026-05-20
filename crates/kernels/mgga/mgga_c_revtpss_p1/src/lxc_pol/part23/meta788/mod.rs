//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta788 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2601;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2602;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta788<F: Float>(t14472: F, t1580: F, t2439: F, t2444: F, t6049: F, t689: F, t136: F, t2457: F, t41011: F, t6048: F, t10504: F, t6071: F, t11007: F, t252: F, t2782: F, t886: F, t14481: F, t1569: F, t18805: F, t41066: F, t10995: F, t122: F, t18796: F, t2466: F) -> (F, F, F, F, F, F, F, F) {
        let (t61400, t61403, t61407, t61411) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2601::<F>(t14472, t1580, t2439, t2444, t6049, t689, t136, t2457, t41011, t6048, t10504, t6071);
        let (t61419, t61422, t61430, t61437) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2602::<F>(t11007, t252, t2782, t6048, t886, t14481, t1569, t18805, t41066, t10995, t122, t18796, t2466);
    (t61400, t61403, t61407, t61411, t61419, t61422, t61430, t61437)
}
