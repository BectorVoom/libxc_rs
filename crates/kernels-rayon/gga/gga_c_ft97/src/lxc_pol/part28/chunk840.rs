//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 840/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk840(t32140: f64, t34476: f64, t2035: f64, t22513: f64, t22623: f64, t22796: f64, t22819: f64, t32169: f64, t32181: f64, t32239: f64, t32241: f64, t32279: f64, t32304: f64, t32313: f64, t34440: f64, t34444: f64, t34451: f64, t34455: f64, t34458: f64, t34461: f64, t34468: f64, t34472: f64, t7867: f64) -> (f64, f64) {
    let t34477 = t32140 * t34476;
    let t34480 = -0.19762785756235085044e-4_f64 * t7867 * t2035 * t34451 - 0.39601100101559655353e-5_f64 * t22796 * t34455 - 0.68116566383613497688e-3_f64 * t22819 * t34458 + t32181 + 0.11352761063935582948e-3_f64 * t22513 * t34461 + 0.25845121844514357744e-4_f64 * t32304 * t34440 + 0.22227677429409423704e-2_f64 * t22623 * t34444 - 0.68246728907663312894e-4_f64 * t32239 * t32241 * t34468 - 0.17608347349624143343e-1_f64 * t32169 * t32140 * t34472 + t32313 + 0.39129660776942540761e-2_f64 * t32279 * t34477;
    (t34477, t34480)
}
