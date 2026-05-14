//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 626/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk626<F: Float>(t9784: F, t2283: F, t2412: F, t128: F, t1910: F, t118: F, t2001: F, t675: F, t2286: F, t1934: F, t1986: F, t1743: F, t649: F, t27: F, t2139: F, t1756: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9785 = 0.53205749866622299248e-5 * t9784;
    let t9786 = t2412 * t2283;
    let t9787 = 0.85129199786595678796e-5 * t9786;
    let t9788 = t128 * t1910;
    let t9789 = t118 * t9788;
    let t9790 = t2001 * t9789;
    let t9791 = t675 * t9790;
    let t9792 = 0.42564599893297839398e-5 * t9791;
    let t9793 = t2412 * t2286;
    let t9794 = 0.25538759935978703638e-4 * t9793;
    let t9795 = t1986 * t1934;
    let t9796 = t675 * t9795;
    let t9797 = 0.12769379967989351819e-4 * t9796;
    let t9798 = t649 * t1743;
    let t9799 = t27 * t9798;
    let t9800 = t2139 * t9799;
    let t9801 = 0.13637330827122670864e-1 * t9800;
    let t9802 = t649 * t1756;
    (t9785, t9787, t9790, t9792, t9794, t9795, t9797, t9799, t9801, t9802)
}
