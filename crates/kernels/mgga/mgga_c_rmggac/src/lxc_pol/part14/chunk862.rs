//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 862/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk862<F: Float>(t7269: F, t8368: F, t7494: F, t8537: F, t34869: F, t34871: F, t34873: F, t34875: F, t34882: F, t34885: F, t34887: F, t34889: F, t34894: F, t38991: F, t38996: F, t38998: F, t39003: F, t39009: F, t39016: F, t39021: F) -> F {
    let t39023 = t8368 * t7269;
    let t39024 = F::cast_from(0.18183107769496894486e-1_f64) * t39023;
    let t39025 = t7494 * t8537;
    let t39027 = -F::cast_from(0.51077519871957407276e-4_f64) * t38991 - F::cast_from(0.1064114997332445985e-4_f64) * t38996 - F::cast_from(0.59590439850616975156e-4_f64) * t38998 + F::cast_from(0.59590439850616975158e-4_f64) * t34869 - F::cast_from(0.59590439850616975158e-4_f64) * t34871 - F::cast_from(0.19863479950205658386e-4_f64) * t34873 - F::cast_from(0.53205749866622299248e-5_f64) * t39003 + F::cast_from(0.99317399751028291929e-5_f64) * t34875 + F::cast_from(0.35913881159970051992e-4_f64) * t39009 + F::cast_from(0.19863479950205658386e-4_f64) * t34882 + F::cast_from(0.74488049813271218947e-4_f64) * t34885 - F::cast_from(0.59590439850616975158e-4_f64) * t34887 - F::cast_from(0.25538759935978703638e-3_f64) * t39016 + F::cast_from(0.19863479950205658386e-4_f64) * t34889 - F::cast_from(0.24829349937757072982e-4_f64) * t34894 + F::cast_from(0.68186654135613354322e-2_f64) * t39021 + t39024 - F::cast_from(0.54549323308490683456e-1_f64) * t39025;
    t39027
}
