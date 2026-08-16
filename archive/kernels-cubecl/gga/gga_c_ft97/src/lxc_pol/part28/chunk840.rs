//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 840/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk840<F: Float>(t32140: F, t34476: F, t2035: F, t22513: F, t22623: F, t22796: F, t22819: F, t32169: F, t32181: F, t32239: F, t32241: F, t32279: F, t32304: F, t32313: F, t34440: F, t34444: F, t34451: F, t34455: F, t34458: F, t34461: F, t34468: F, t34472: F, t7867: F) -> (F, F) {
    let t34477 = t32140 * t34476;
    let t34480 = -F::cast_from(0.19762785756235085044e-4_f64) * t7867 * t2035 * t34451 - F::cast_from(0.39601100101559655353e-5_f64) * t22796 * t34455 - F::cast_from(0.68116566383613497688e-3_f64) * t22819 * t34458 + t32181 + F::cast_from(0.11352761063935582948e-3_f64) * t22513 * t34461 + F::cast_from(0.25845121844514357744e-4_f64) * t32304 * t34440 + F::cast_from(0.22227677429409423704e-2_f64) * t22623 * t34444 - F::cast_from(0.68246728907663312894e-4_f64) * t32239 * t32241 * t34468 - F::cast_from(0.17608347349624143343e-1_f64) * t32169 * t32140 * t34472 + t32313 + F::cast_from(0.39129660776942540761e-2_f64) * t32279 * t34477;
    (t34477, t34480)
}
