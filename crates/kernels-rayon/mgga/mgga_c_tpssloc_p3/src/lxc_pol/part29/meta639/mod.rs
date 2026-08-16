//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2100;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2101;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2102;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2103;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2104;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta639(t25041: f64, t87049: f64, t215: f64, t6581: f64, t252: f64, t81613: f64, t13224: f64, t23056: f64, t13352: f64, t25242: f64, t6579: f64, t25245: f64, t82031: f64, t81575: f64, t25038: f64, t4282: f64, t6646: f64, t9647: f64, t25251: f64, t23012: f64, t7529: f64, t13380: f64, t22986: f64, t2647: f64, t13377: f64, t1880: f64, t1894: f64, t214: f64, t22984: f64, t22992: f64, t22993: f64, t23009: f64, t25297: f64, t2617: f64, t4166: f64, t4234: f64, t812: f64, t81571: f64, t81592: f64, t1888: f64, t232: f64, t47448: f64, t23110: f64, t23185: f64, t25241: f64, t25248: f64, t25249: f64, t2553: f64, t1519: f64, t2631: f64, t1484: f64, t852: f64, t776: f64, t13393: f64, t22996: f64, t81595: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87050, t87052, t87055, t87059, t87067, t87068) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2100(t25041, t87049, t215, t6581, t252, t81613, t13224, t23056, t13352, t25242, t6579, t25245, t82031);
        let (t87073, t87076, t87078, t87080, t87084) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2101(t81575, t25038, t4282, t6646, t9647, t25251, t87049, t23012, t7529, t13380, t22986, t2647);
        let t87094 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2102(t13377, t1880, t1894, t214, t22984, t22992, t22993, t23009, t25297, t2617, t4166, t4234, t812, t81571, t81592, t87055, t87059, t87067, t87068, t87073, t87076, t87078, t87080, t87084);
        let (t87097, t87101, t87104, t87106) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2103(t1888, t232, t47448, t6646, t23110, t23185, t25241, t25038, t25248, t25249, t2553, t1519, t2631);
        let (t87109, t87111, t87114, t87117, t87119) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2104(t1888, t232, t6646, t87106, t1484, t852, t25038, t25248, t776, t13393, t22996, t81595);
    (t87050, t87052, t87094, t87097, t87101, t87104, t87106, t87109, t87111, t87114, t87117, t87119)
}
