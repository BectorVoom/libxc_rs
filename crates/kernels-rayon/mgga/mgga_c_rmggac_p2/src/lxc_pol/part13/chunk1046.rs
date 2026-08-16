//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1046/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1046(t38998: f64, t39023: f64, t39025: f64, t34869: f64, t34871: f64, t34873: f64, t34875: f64, t34882: f64, t34885: f64, t34887: f64, t34889: f64, t34894: f64, t38996: f64, t39003: f64, t39009: f64, t39016: f64, t39021: f64) -> f64 {
    let t42806 = 0.11918087970123395032e-3_f64 * t38998;
    let t42820 = 0.36366215538993788974e-1_f64 * t39023;
    let t42821 = 0.10909864661698136692e0_f64 * t39025;
    let t42822 = -0.212822999466489197e-4_f64 * t38996 - t42806 + 0.11918087970123395032e-3_f64 * t34869 - 0.11918087970123395032e-3_f64 * t34871 - 0.39726959900411316772e-4_f64 * t34873 - 0.1064114997332445985e-4_f64 * t39003 + 0.19863479950205658386e-4_f64 * t34875 + 0.71827762319940103988e-4_f64 * t39009 + 0.39726959900411316772e-4_f64 * t34882 + 0.14897609962654243789e-3_f64 * t34885 - 0.11918087970123395032e-3_f64 * t34887 - 0.5107751987195740728e-3_f64 * t39016 + 0.39726959900411316772e-4_f64 * t34889 - 0.49658699875514145964e-4_f64 * t34894 + 0.13637330827122670865e-1_f64 * t39021 + t42820 - t42821;
    t42822
}
