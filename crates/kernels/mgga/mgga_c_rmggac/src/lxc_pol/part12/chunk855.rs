//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 855/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk855<F: Float>(t1364: F, t34807: F, t34810: F, t34820: F, t38887: F, t38889: F, t38899: F, t38901: F, t38908: F, t38913: F, t38918: F, t38922: F, t38926: F, t38932: F, t38934: F, t38938: F, t4041: F, t5184: F, t5218: F, t665: F, t8399: F, t903: F) -> F {
    let t38940 = t38887 + F::new(0.81300399444200075504e-3) * t38889 - F::new(0.23948483403727617128e0) * t1364 * t665 * t5184 - F::new(0.23948483403727617128e0) * t4041 * t8399 + F::new(0.17961362552795712846e0) * t903 * t665 * t5218 + F::new(0.10227998120342003148e-1) * t38899 - t34807 - F::new(0.13637330827122670864e-1) * t38901 - F::new(0.66671395154821946448e-1) * t34810 - F::new(0.18183107769496894486e-1) * t34820 - F::new(0.31923449919973379548e-4) * t38908 - F::new(0.31923449919973379548e-4) * t38913 - F::new(0.25538759935978703638e-4) * t38918 + F::new(0.76616279807936110914e-4) * t38922 - F::new(0.51077519871957407277e-4) * t38926 - F::new(0.12769379967989351819e-4) * t38932 - F::new(0.59590439850616975156e-4) * t38934 - F::new(0.10215503974391481455e-3) * t38938;
    t38940
}
