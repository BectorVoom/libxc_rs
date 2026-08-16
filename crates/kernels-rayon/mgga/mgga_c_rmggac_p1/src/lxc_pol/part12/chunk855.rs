//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 855/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk855(t1364: f64, t34807: f64, t34810: f64, t34820: f64, t38887: f64, t38889: f64, t38899: f64, t38901: f64, t38908: f64, t38913: f64, t38918: f64, t38922: f64, t38926: f64, t38932: f64, t38934: f64, t38938: f64, t4041: f64, t5184: f64, t5218: f64, t665: f64, t8399: f64, t903: f64) -> f64 {
    let t38940 = t38887 + 0.81300399444200075504e-3_f64 * t38889 - 0.23948483403727617128e0_f64 * t1364 * t665 * t5184 - 0.23948483403727617128e0_f64 * t4041 * t8399 + 0.17961362552795712846e0_f64 * t903 * t665 * t5218 + 0.10227998120342003148e-1_f64 * t38899 - t34807 - 0.13637330827122670864e-1_f64 * t38901 - 0.66671395154821946448e-1_f64 * t34810 - 0.18183107769496894486e-1_f64 * t34820 - 0.31923449919973379548e-4_f64 * t38908 - 0.31923449919973379548e-4_f64 * t38913 - 0.25538759935978703638e-4_f64 * t38918 + 0.76616279807936110914e-4_f64 * t38922 - 0.51077519871957407277e-4_f64 * t38926 - 0.12769379967989351819e-4_f64 * t38932 - 0.59590439850616975156e-4_f64 * t38934 - 0.10215503974391481455e-3_f64 * t38938;
    t38940
}
