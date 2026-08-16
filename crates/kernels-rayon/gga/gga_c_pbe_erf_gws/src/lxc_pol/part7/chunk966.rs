//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 966/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk966(t1403: f64, t1407: f64, t1648: f64, t1663: f64, t17577: f64, t1759: f64, t17846: f64, t17850: f64, t17852: f64, t17872: f64, t17875: f64, t1804: f64, t1827: f64, t186: f64, t1866: f64, t198: f64, t2660: f64, t4891: f64, t4982: f64, t5335: f64, t5543: f64, t5551: f64, t561: f64, t587: f64, t612: f64) -> f64 {
    let t17877 = -4.0_f64 / 15.0_f64 * t561 * t186 * t198 * t17577 + 32.0_f64 / 15.0_f64 * t17846 + 16.0_f64 / 15.0_f64 * t2660 * t5335 - 64.0_f64 / 45.0_f64 * t17850 + 16.0_f64 / 9.0_f64 * t587 * t17852 * t1759 * t1804 - 8.0_f64 / 15.0_f64 * t4982 * t612 + 32.0_f64 / 15.0_f64 * t1648 * t5551 - 8.0_f64 / 15.0_f64 * t587 * t1827 * t4891 * t1407 - 8.0_f64 / 9.0_f64 * t587 * t5543 * t1866 * t1663 * t1403 - 32.0_f64 / 27.0_f64 * t17872 - 64.0_f64 / 45.0_f64 * t17875;
    t17877
}
