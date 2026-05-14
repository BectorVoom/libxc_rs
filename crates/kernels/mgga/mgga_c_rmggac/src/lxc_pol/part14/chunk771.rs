//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 771/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk771<F: Float>(t1971: F, t3351: F, t3924: F, t5223: F, t623: F, t7262: F, t7265: F, t7269: F, t8368: F, t7494: F, t8537: F, t34869: F, t34871: F, t34873: F, t34875: F, t34882: F, t34885: F, t34887: F, t34889: F, t34894: F, t38991: F, t38996: F, t38998: F, t39003: F, t39009: F) -> (F,) {
    let t39016 = t3351 * t1971 * t3924 * t5223;
    let t39020 = t623 * t7262;
    let t39021 = t39020 * t7265;
    let t39023 = t8368 * t7269;
    let t39024 = 0.18183107769496894486e-1 * t39023;
    let t39025 = t7494 * t8537;
    let t39027 = -0.51077519871957407276e-4 * t38991 - 0.1064114997332445985e-4 * t38996 - 0.59590439850616975156e-4 * t38998 + 0.59590439850616975158e-4 * t34869 - 0.59590439850616975158e-4 * t34871 - 0.19863479950205658386e-4 * t34873 - 0.53205749866622299248e-5 * t39003 + 0.99317399751028291929e-5 * t34875 + 0.35913881159970051992e-4 * t39009 + 0.19863479950205658386e-4 * t34882 + 0.74488049813271218947e-4 * t34885 - 0.59590439850616975158e-4 * t34887 - 0.25538759935978703638e-3 * t39016 + 0.19863479950205658386e-4 * t34889 - 0.24829349937757072982e-4 * t34894 + 0.68186654135613354322e-2 * t39021 + t39024 - 0.54549323308490683456e-1 * t39025;
    (t39027,)
}
