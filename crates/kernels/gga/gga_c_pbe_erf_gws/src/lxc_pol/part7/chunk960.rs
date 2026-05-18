//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 960/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk960<F: Float>(t5463: F, t649: F, t1816: F, t639: F, t1726: F, t1783: F, t1683: F, t1798: F, t5343: F, t185: F, t5274: F, t582: F) -> (F, F, F, F, F) {
    let t17791 = t5463 * t649;
    let t17793 = t639 * t17791 * t1816;
    let t17794 = F::new(32.0) / F::new(135.0) * t17793;
    let t17796 = F::new(8.0) / F::new(5.0) * t1783 * t1726;
    let t17797 = t1783 * t1683;
    let t17798 = F::new(32.0) / F::new(15.0) * t17797;
    let t17799 = t5343 * t1798;
    let t17800 = F::new(32.0) / F::new(15.0) * t17799;
    let t17802 = t185 * t582 * t5274;
    (t17794, t17796, t17798, t17800, t17802)
}
