//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 831/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk831<F: Float>(t17072: F, t1893: F, t5467: F, t5207: F, t579: F, t5563: F, t5179: F, t1778: F, t1783: F, t17058: F, t17063: F, t17067: F, t17069: F, t17071: F, t1661: F, t16669: F, t5294: F, t587: F) -> (F, F, F, F, F, F, F, F) {
    let t17073 = 64.0 / 45.0 * t17072;
    let t17075 = 32.0 / 15.0 * t5467 * t1893;
    let t17076 = t579 * t5207;
    let t17077 = 16.0 / 45.0 * t17076;
    let t17078 = t579 * t5563;
    let t17079 = 32.0 / 15.0 * t17078;
    let t17081 = 16.0 / 5.0 * t579 * t5179;
    let t17082 = t1783 * t1778;
    let t17083 = 16.0 / 45.0 * t17082;
    let t17084 = -t17058 - t17063 + t17067 - t17069 - t17071 + t17073 - t17075 + t17077 + t17079 - t17081 - t17083;
    let t17090 = 16.0 / 3.0 * t587 * t1661 * t5294 * t16669;
    (t17073, t17075, t17077, t17079, t17081, t17083, t17084, t17090)
}
