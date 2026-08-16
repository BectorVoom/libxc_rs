//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 663/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk663(t12223: f64, t550: f64, t549: f64, t1890: f64, t3720: f64, t590: f64, t3726: f64, t10941: f64, t10944: f64, t10953: f64, t10963: f64, t10966: f64, t10971: f64, t10975: f64, t10977: f64, t10980: f64, t10983: f64, t10988: f64, t10990: f64, t10993: f64, t1966: f64, t1991: f64, t2033: f64) -> (f64, f64, f64) {
    let t12236 = t550 * t12223;
    let t12237 = t549 * t12236;
    let t12240 = t1890 * t3720;
    let t12241 = t12240 * t590;
    let t12244 = t3726 * t590;
    let t12247 = -t10941 + 0.39722766613167140743e-1_f64 * t2033 * t12237 + t10944 - t10953 + t10963 - t10966 - 0.51123901271894332902e0_f64 * t1966 * t12241 + 0.51123901271894332902e0_f64 * t1991 * t12244 - t10971 - t10975 + t10977 + t10980 + t10983 - t10988 - t10990 - t10993;
    (t12236, t12240, t12247)
}
