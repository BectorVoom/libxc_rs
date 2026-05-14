//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 919/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk919<F: Float>(t333: F, t3351: F, t511: F, t9210: F, t9211: F, t352: F, t515: F, t2604: F, t36610: F, t36613: F, t40811: F, t41736: F, t41739: F, t41745: F, t41747: F, t41751: F, t41755: F, t41760: F, t41763: F, t41767: F, t41772: F, t41774: F, t739: F, t8988: F) -> (F,) {
    let t41779 = t3351 * t9210 * t511 * t9211 * t333;
    let t41784 = t3351 * t9210 * t515 * t9211 * t352;
    let t41788 = -t41736 - 0.20455996240684006297e-1 * t41739 - 0.59871208509319042821e-1 * t739 * t40811 - 0.85129199786595678796e-5 * t41745 + 0.1064114997332445985e-4 * t41747 + 0.25538759935978703638e-4 * t41751 - 0.25538759935978703638e-4 * t41755 - 0.1064114997332445985e-4 * t41760 + t41763 + 0.59590439850616975158e-4 * t36610 - 0.27933018679976707105e-4 * t36613 + 0.33105799917009430643e-4 * t41767 - 0.42564599893297839398e-5 * t41772 - 0.99317399751028291927e-4 * t41774 - 0.51077519871957407277e-4 * t41779 - 0.17025839957319135759e-4 * t41784 - 0.11974241701863808564e0 * t2604 * t8988;
    (t41788,)
}
