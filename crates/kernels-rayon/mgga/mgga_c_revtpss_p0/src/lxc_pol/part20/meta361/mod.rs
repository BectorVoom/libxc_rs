//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1311;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1312;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1313;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta361(t10069: f64, t10934: f64, t10518: f64, t10542: f64, t39419: f64, t39422: f64, t39424: f64, t39426: f64, t39429: f64, t39432: f64, t39434: f64, t39437: f64, t39439: f64, t39442: f64, t39483: f64, t10612: f64, t2398: f64, t2434: f64, t2626: f64, t2629: f64, t676: f64, t9425: f64, t2567: f64, t2576: f64, t2582: f64, t2577: f64, t268: f64, t9326: f64, t39520: f64, t39528: f64, t39531: f64, t39534: f64, t39537: f64, t39540: f64, t215: f64, t2581: f64, t2585: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39726, t39731, t39736) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1311(t10069, t10934, t10518, t10542, t39419, t39422, t39424, t39426, t39429, t39432, t39434, t39437, t39439, t39442, t39483);
        let (t39738, t39739, t39741, t39742, t39744, t39747) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1312(t10612, t2398, t2434, t2626, t2629, t676, t9425, t2567, t2576, t2582);
        let t39750 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1313(t2577, t268, t9326);
        let (t39751, t39756) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1314(t39520, t39528, t39531, t39534, t39537, t39540, t39738, t39741, t39744, t39747, t39750, t215, t2581, t2585, t268);
    (t39726, t39731, t39736, t39738, t39739, t39741, t39742, t39744, t39747, t39750, t39751, t39756)
}
