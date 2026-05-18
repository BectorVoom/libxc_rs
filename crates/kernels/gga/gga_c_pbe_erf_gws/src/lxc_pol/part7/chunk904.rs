//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 904/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk904<F: Float>(t17068: F, t1627: F, t4998: F, t1631: F, t5467: F, t1893: F, t5207: F, t579: F, t5563: F, t5179: F, t1778: F, t1783: F) -> (F, F, F, F, F, F, F, F) {
    let t17069 = F::new(32.0) / F::new(135.0) * t17068;
    let t17070 = t1627 * t4998;
    let t17071 = F::new(64.0) / F::new(45.0) * t17070;
    let t17072 = t5467 * t1631;
    let t17073 = F::new(64.0) / F::new(45.0) * t17072;
    let t17075 = F::new(32.0) / F::new(15.0) * t5467 * t1893;
    let t17076 = t579 * t5207;
    let t17077 = F::new(16.0) / F::new(45.0) * t17076;
    let t17078 = t579 * t5563;
    let t17079 = F::new(32.0) / F::new(15.0) * t17078;
    let t17081 = F::new(16.0) / F::new(5.0) * t579 * t5179;
    let t17082 = t1783 * t1778;
    (t17069, t17071, t17073, t17075, t17077, t17079, t17081, t17082)
}
