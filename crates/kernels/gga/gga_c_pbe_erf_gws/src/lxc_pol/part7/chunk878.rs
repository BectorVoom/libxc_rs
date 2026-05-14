//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 878/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk878<F: Float>(t1882: F, t5379: F, t1627: F, t5138: F, t1661: F, t1802: F, t5480: F, t649: F, t5523: F, t639: F, t4934: F, t5038: F, t1403: F, t1407: F, t1648: F, t1663: F, t17577: F, t1759: F, t1804: F, t1827: F, t186: F, t1866: F, t198: F, t2660: F, t4891: F, t4982: F, t5335: F, t5543: F, t5551: F, t561: F, t587: F, t612: F) -> (F,) {
    let t17846 = t5379 * t1882;
    let t17850 = t1627 * t5138;
    let t17852 = t1661 * t1802;
    let t17870 = t5480 * t649;
    let t17872 = t639 * t17870 * t5523;
    let t17875 = t639 * t4934 * t5038;
    let t17877 = -4.0 / 15.0 * t561 * t186 * t198 * t17577 + 32.0 / 15.0 * t17846 + 16.0 / 15.0 * t2660 * t5335 - 64.0 / 45.0 * t17850 + 16.0 / 9.0 * t587 * t17852 * t1759 * t1804 - 8.0 / 15.0 * t4982 * t612 + 32.0 / 15.0 * t1648 * t5551 - 8.0 / 15.0 * t587 * t1827 * t4891 * t1407 - 8.0 / 9.0 * t587 * t5543 * t1866 * t1663 * t1403 - 32.0 / 27.0 * t17872 - 64.0 / 45.0 * t17875;
    (t17877,)
}
