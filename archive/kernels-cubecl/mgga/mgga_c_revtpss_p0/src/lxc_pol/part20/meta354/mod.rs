//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta354 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1290;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1291;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1292;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1293;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1294;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta354<F: Float>(t2501: F, t39494: F, t2698: F, t685: F, t684: F, t125: F, t123: F, t128: F, t121: F, t268: F, t8779: F, t588: F, t9295: F, t2508: F, t692: F, t124: F, t138: F, t239: F, t39490: F, t39492: F, t682: F, t701: F, t198: F, t39419: F, t39422: F, t39424: F, t39426: F, t39429: F, t39432: F, t39434: F, t39437: F, t39439: F, t39442: F, t39476: F, t39483: F, t765: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39495, t39497) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1290::<F>(t2501, t39494, t2698, t685);
        let (t39498, t39500, t39501) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1291::<F>(t39497, t684, t125, t2698, t123);
        let (t39506, t39508, t39510, t39512, t39515) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1292::<F>(t128, t121, t268, t8779, t588, t9295, t2508, t39494, t39497, t692, t124, t138, t239);
        let t39520 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1293::<F>(t39490, t39492, t39495, t39498, t39501, t39506, t39508, t39510, t39512, t39515, t682, t701);
        let t39521 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1294::<F>(t198, t39419, t39422, t39424, t39426, t39429, t39432, t39434, t39437, t39439, t39442, t39476, t39483, t39520, t765);
    (t39495, t39497, t39498, t39500, t39501, t39506, t39508, t39510, t39512, t39515, t39520, t39521)
}
