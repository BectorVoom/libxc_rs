//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1096/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1096(t13283: f64, t2028: f64, t47866: f64, t47868: f64, t47872: f64, t47874: f64, t47876: f64, t47881: f64, t47883: f64, t47885: f64, t47887: f64, t47889: f64, t47891: f64, t47898: f64, t47903: f64, t47908: f64, t47913: f64, t47918: f64, t47923: f64) -> f64 {
    let t47925 = 0.85129199786595678796e-5_f64 * t47866 - 0.42564599893297839398e-5_f64 * t47868 - 0.1064114997332445985e-4_f64 * t47872 - 0.85129199786595678796e-5_f64 * t47874 - 0.31923449919973379548e-4_f64 * t47876 + 0.42564599893297839398e-5_f64 * t47881 - 0.42564599893297839398e-5_f64 * t47883 - 0.10227998120342003148e-1_f64 * t47885 + 0.13637330827122670864e-1_f64 * t47887 + 0.68186654135613354322e-2_f64 * t47889 + 0.20455996240684006296e-1_f64 * t47891 - 0.59871208509319042821e-1_f64 * t13283 * t2028 + 0.31923449919973379548e-4_f64 * t47898 - 0.63846899839946759096e-4_f64 * t47903 + 0.95770349759920138644e-4_f64 * t47908 + 0.31923449919973379548e-4_f64 * t47913 - 0.31923449919973379548e-4_f64 * t47918 + 0.25538759935978703638e-4_f64 * t47923;
    t47925
}
