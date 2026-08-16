//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk645;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk646;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk647;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta115(t225: f64, t4075: f64, t1429: f64, t2435: f64, t1428: f64, t2777: f64, t2439: f64, t1385: f64, t136: f64, t555: f64, t2457: f64, t3964: f64, t786: f64, t1432: f64, t1433: f64, t2470: f64, t3999: f64, t198: f64, t531: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4076, t4082, t4083, t4085, t4086) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk645(t225, t4075, t1429, t2435, t1428, t2777, t2439, t1385);
        let (t4096, t4099, t4100, t4101) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk646(t136, t555, t2457, t3964, t4086, t786);
        let (t4113, t4114) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk647(t1432, t1433, t2470, t3999, t555);
        let t4139 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk648(t198, t531);
    (t4076, t4082, t4083, t4085, t4086, t4096, t4099, t4100, t4101, t4113, t4114, t4139)
}
