//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 817/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk817<F: Float>(t1464: F, t671: F, t1457: F, t4892: F, t5129: F, t587: F, t1820: F, t4919: F, t5125: F, t1663: F, t1821: F, t4352: F, t562: F, t1680: F, t1740: F, t5516: F, t612: F) -> (F, F, F, F, F, F, F) {
    let t16876 = 0.44134814814814814812e-2 * t1464 * t671;
    let t16877 = t1457 * t671;
    let t16880 = t587 * t5129 * t4892;
    let t16881 = 32.0 / 45.0 * t16880;
    let t16883 = t1820 * t5125 * t4919;
    let t16884 = 128.0 / 45.0 * t16883;
    let t16889 = 64.0 / 15.0 * t1820 * t1821 * t562 * t1663 * t4352;
    let t16890 = t1680 * t1740;
    let t16891 = 32.0 / 15.0 * t16890;
    let t16893 = 16.0 / 5.0 * t5516 * t612;
    (t16876, t16877, t16881, t16884, t16889, t16891, t16893)
}
