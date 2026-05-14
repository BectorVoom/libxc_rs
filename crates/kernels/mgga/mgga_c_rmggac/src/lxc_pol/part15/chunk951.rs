//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 951/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk951<F: Float>(t13283: F, t2028: F, t47866: F, t47868: F, t47872: F, t47874: F, t47876: F, t47881: F, t47883: F, t47885: F, t47887: F, t47889: F, t47891: F, t47898: F, t47903: F, t47908: F, t47913: F, t47918: F, t47923: F) -> (F,) {
    let t47925 = 0.85129199786595678796e-5 * t47866 - 0.42564599893297839398e-5 * t47868 - 0.1064114997332445985e-4 * t47872 - 0.85129199786595678796e-5 * t47874 - 0.31923449919973379548e-4 * t47876 + 0.42564599893297839398e-5 * t47881 - 0.42564599893297839398e-5 * t47883 - 0.10227998120342003148e-1 * t47885 + 0.13637330827122670864e-1 * t47887 + 0.68186654135613354322e-2 * t47889 + 0.20455996240684006296e-1 * t47891 - 0.59871208509319042821e-1 * t13283 * t2028 + 0.31923449919973379548e-4 * t47898 - 0.63846899839946759096e-4 * t47903 + 0.95770349759920138644e-4 * t47908 + 0.31923449919973379548e-4 * t47913 - 0.31923449919973379548e-4 * t47918 + 0.25538759935978703638e-4 * t47923;
    (t47925,)
}
