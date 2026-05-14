//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 950/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk950<F: Float>(t7780: F, t7784: F, t27055: F, t7788: F, t1281: F, t7807: F, t2201: F, t3668: F, t8027: F, t911: F, t2167: F, t4527: F, t1876: F, t914: F, t2169: F, t7673: F, t8024: F) -> (F, F, F, F, F, F, F, F) {
    let t27080 = t7780 * t7784;
    let t27087 = t7788 * t27055;
    let t27100 = t7807 * t1281;
    let t27141 = t2201 * t3668;
    let t27731 = t911 * t8027;
    let t27733 = t4527 * t2167;
    let t27734 = t914 * t1876;
    let t27735 = t2169 * t27734;
    let t27737 = t7673 * t8024;
    (t27080, t27087, t27100, t27141, t27731, t27733, t27735, t27737)
}
