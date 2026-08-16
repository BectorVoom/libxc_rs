//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 862/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk862(t7269: f64, t8368: f64, t7494: f64, t8537: f64, t34869: f64, t34871: f64, t34873: f64, t34875: f64, t34882: f64, t34885: f64, t34887: f64, t34889: f64, t34894: f64, t38991: f64, t38996: f64, t38998: f64, t39003: f64, t39009: f64, t39016: f64, t39021: f64) -> f64 {
    let t39023 = t8368 * t7269;
    let t39024 = 0.18183107769496894486e-1_f64 * t39023;
    let t39025 = t7494 * t8537;
    let t39027 = -0.51077519871957407276e-4_f64 * t38991 - 0.1064114997332445985e-4_f64 * t38996 - 0.59590439850616975156e-4_f64 * t38998 + 0.59590439850616975158e-4_f64 * t34869 - 0.59590439850616975158e-4_f64 * t34871 - 0.19863479950205658386e-4_f64 * t34873 - 0.53205749866622299248e-5_f64 * t39003 + 0.99317399751028291929e-5_f64 * t34875 + 0.35913881159970051992e-4_f64 * t39009 + 0.19863479950205658386e-4_f64 * t34882 + 0.74488049813271218947e-4_f64 * t34885 - 0.59590439850616975158e-4_f64 * t34887 - 0.25538759935978703638e-3_f64 * t39016 + 0.19863479950205658386e-4_f64 * t34889 - 0.24829349937757072982e-4_f64 * t34894 + 0.68186654135613354322e-2_f64 * t39021 + t39024 - 0.54549323308490683456e-1_f64 * t39025;
    t39027
}
