//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1046/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1046<F: Float>(t38998: F, t39023: F, t39025: F, t34869: F, t34871: F, t34873: F, t34875: F, t34882: F, t34885: F, t34887: F, t34889: F, t34894: F, t38996: F, t39003: F, t39009: F, t39016: F, t39021: F) -> F {
    let t42806 = F::cast_from(0.11918087970123395032e-3_f64) * t38998;
    let t42820 = F::cast_from(0.36366215538993788974e-1_f64) * t39023;
    let t42821 = F::cast_from(0.10909864661698136692e0_f64) * t39025;
    let t42822 = -F::cast_from(0.212822999466489197e-4_f64) * t38996 - t42806 + F::cast_from(0.11918087970123395032e-3_f64) * t34869 - F::cast_from(0.11918087970123395032e-3_f64) * t34871 - F::cast_from(0.39726959900411316772e-4_f64) * t34873 - F::cast_from(0.1064114997332445985e-4_f64) * t39003 + F::cast_from(0.19863479950205658386e-4_f64) * t34875 + F::cast_from(0.71827762319940103988e-4_f64) * t39009 + F::cast_from(0.39726959900411316772e-4_f64) * t34882 + F::cast_from(0.14897609962654243789e-3_f64) * t34885 - F::cast_from(0.11918087970123395032e-3_f64) * t34887 - F::cast_from(0.5107751987195740728e-3_f64) * t39016 + F::cast_from(0.39726959900411316772e-4_f64) * t34889 - F::cast_from(0.49658699875514145964e-4_f64) * t34894 + F::cast_from(0.13637330827122670865e-1_f64) * t39021 + t42820 - t42821;
    t42822
}
