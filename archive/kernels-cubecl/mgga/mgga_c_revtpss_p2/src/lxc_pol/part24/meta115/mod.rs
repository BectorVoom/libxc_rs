//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk645;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk646;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk647;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta115<F: Float>(t225: F, t4075: F, t1429: F, t2435: F, t1428: F, t2777: F, t2439: F, t1385: F, t136: F, t555: F, t2457: F, t3964: F, t786: F, t1432: F, t1433: F, t2470: F, t3999: F, t198: F, t531: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t4076, t4082, t4083, t4085, t4086) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk645::<F>(t225, t4075, t1429, t2435, t1428, t2777, t2439, t1385);
        let (t4096, t4099, t4100, t4101) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk646::<F>(t136, t555, t2457, t3964, t4086, t786);
        let (t4113, t4114) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk647::<F>(t1432, t1433, t2470, t3999, t555);
        let t4139 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk648::<F>(t198, t531);
    (t4076, t4082, t4083, t4085, t4086, t4096, t4099, t4100, t4101, t4113, t4114, t4139)
}
