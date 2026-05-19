//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1096/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1096<F: Float>(t13283: F, t2028: F, t47866: F, t47868: F, t47872: F, t47874: F, t47876: F, t47881: F, t47883: F, t47885: F, t47887: F, t47889: F, t47891: F, t47898: F, t47903: F, t47908: F, t47913: F, t47918: F, t47923: F) -> F {
    let t47925 = F::cast_from(0.85129199786595678796e-5_f64) * t47866 - F::cast_from(0.42564599893297839398e-5_f64) * t47868 - F::cast_from(0.1064114997332445985e-4_f64) * t47872 - F::cast_from(0.85129199786595678796e-5_f64) * t47874 - F::cast_from(0.31923449919973379548e-4_f64) * t47876 + F::cast_from(0.42564599893297839398e-5_f64) * t47881 - F::cast_from(0.42564599893297839398e-5_f64) * t47883 - F::cast_from(0.10227998120342003148e-1_f64) * t47885 + F::cast_from(0.13637330827122670864e-1_f64) * t47887 + F::cast_from(0.68186654135613354322e-2_f64) * t47889 + F::cast_from(0.20455996240684006296e-1_f64) * t47891 - F::cast_from(0.59871208509319042821e-1_f64) * t13283 * t2028 + F::cast_from(0.31923449919973379548e-4_f64) * t47898 - F::cast_from(0.63846899839946759096e-4_f64) * t47903 + F::cast_from(0.95770349759920138644e-4_f64) * t47908 + F::cast_from(0.31923449919973379548e-4_f64) * t47913 - F::cast_from(0.31923449919973379548e-4_f64) * t47918 + F::cast_from(0.25538759935978703638e-4_f64) * t47923;
    t47925
}
