//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 649/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk649(t5110: f64, t5111: f64, t186: f64, t211: f64, t1672: f64, t618: f64, t616: f64, t1783: f64, t663: f64, t4937: f64, t4984: f64, t4986: f64, t4987: f64, t4990: f64, t4994: f64, t4997: f64, t5000: f64, t5007: f64, t5012: f64, t5017: f64, t5021: f64, t5101: f64, t5104: f64, t5107: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5112 = t5110 * t5111;
    let t5113 = t186 * t5112;
    let t5115 = 4.0_f64 / 5.0_f64 * t211 * t5113;
    let t5116 = t1672 * t618;
    let t5117 = t616 * t5116;
    let t5118 = 8.0_f64 / 45.0_f64 * t5117;
    let t5120 = 4.0_f64 / 5.0_f64 * t1783 * t663;
    let t5121 = t4937 + t4984 + t4986 - 2.0_f64 / 15.0_f64 * t4987 - t4990 - t4994 - t4997 - t5000 - t5007 - t5012 - t5017 - t5021 - t5101 + t5104 + t5107 - t5115 - t5118 - t5120;
    (t5112, t5113, t5115, t5116, t5118, t5120, t5121)
}
