//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1311;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1312;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1313;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta361<F: Float>(t10069: F, t10934: F, t10518: F, t10542: F, t39419: F, t39422: F, t39424: F, t39426: F, t39429: F, t39432: F, t39434: F, t39437: F, t39439: F, t39442: F, t39483: F, t10612: F, t2398: F, t2434: F, t2626: F, t2629: F, t676: F, t9425: F, t2567: F, t2576: F, t2582: F, t2577: F, t268: F, t9326: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F, t215: F, t2581: F, t2585: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39726, t39731, t39736) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1311::<F>(t10069, t10934, t10518, t10542, t39419, t39422, t39424, t39426, t39429, t39432, t39434, t39437, t39439, t39442, t39483);
        let (t39738, t39739, t39741, t39742, t39744, t39747) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1312::<F>(t10612, t2398, t2434, t2626, t2629, t676, t9425, t2567, t2576, t2582);
        let t39750 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1313::<F>(t2577, t268, t9326);
        let (t39751, t39756) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1314::<F>(t39520, t39528, t39531, t39534, t39537, t39540, t39738, t39741, t39744, t39747, t39750, t215, t2581, t2585, t268);
    (t39726, t39731, t39736, t39738, t39739, t39741, t39742, t39744, t39747, t39750, t39751, t39756)
}
