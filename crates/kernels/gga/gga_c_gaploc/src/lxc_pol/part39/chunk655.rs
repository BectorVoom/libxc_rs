//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 655/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk655<F: Float>(t12223: F, t550: F, t549: F, t1890: F, t3720: F, t590: F, t3726: F, t10941: F, t10944: F, t10953: F, t10963: F, t10966: F, t10971: F, t10975: F, t10977: F, t10980: F, t10983: F, t10988: F, t10990: F, t10993: F, t1966: F, t1991: F, t2033: F) -> (F, F, F) {
    let t12236 = t550 * t12223;
    let t12237 = t549 * t12236;
    let t12240 = t1890 * t3720;
    let t12241 = t12240 * t590;
    let t12244 = t3726 * t590;
    let t12247 = -t10941 + 0.39722766613167140743e-1 * t2033 * t12237 + t10944 - t10953 + t10963 - t10966 - 0.51123901271894332902e0 * t1966 * t12241 + 0.51123901271894332902e0 * t1991 * t12244 - t10971 - t10975 + t10977 + t10980 + t10983 - t10988 - t10990 - t10993;
    (t12236, t12240, t12247)
}
