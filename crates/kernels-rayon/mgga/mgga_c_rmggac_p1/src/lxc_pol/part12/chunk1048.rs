//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1048/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1048(t3351: f64, t352: f64, t515: f64, t9210: f64, t9211: f64, t2604: f64, t36610: f64, t36613: f64, t40811: f64, t41736: f64, t41739: f64, t41745: f64, t41747: f64, t41751: f64, t41755: f64, t41760: f64, t41763: f64, t41767: f64, t41772: f64, t41774: f64, t41779: f64, t739: f64, t8988: f64) -> f64 {
    let t41784 = t3351 * t9210 * t515 * t9211 * t352;
    let t41788 = -t41736 - 0.20455996240684006297e-1_f64 * t41739 - 0.59871208509319042821e-1_f64 * t739 * t40811 - 0.85129199786595678796e-5_f64 * t41745 + 0.1064114997332445985e-4_f64 * t41747 + 0.25538759935978703638e-4_f64 * t41751 - 0.25538759935978703638e-4_f64 * t41755 - 0.1064114997332445985e-4_f64 * t41760 + t41763 + 0.59590439850616975158e-4_f64 * t36610 - 0.27933018679976707105e-4_f64 * t36613 + 0.33105799917009430643e-4_f64 * t41767 - 0.42564599893297839398e-5_f64 * t41772 - 0.99317399751028291927e-4_f64 * t41774 - 0.51077519871957407277e-4_f64 * t41779 - 0.17025839957319135759e-4_f64 * t41784 - 0.11974241701863808564e0_f64 * t2604 * t8988;
    t41788
}
