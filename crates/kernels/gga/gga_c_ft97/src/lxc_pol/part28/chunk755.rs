//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 755/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk755<F: Float>(t7318: F, t938: F, t32152: F, t6441: F, t34434: F, t7195: F, t32233: F, t6449: F, t32242: F, t930: F, t378: F, t1642: F, t925: F, t32140: F, t2035: F, t22513: F, t22623: F, t22796: F, t22819: F, t32169: F, t32181: F, t32239: F, t32241: F, t32279: F, t32304: F, t32313: F, t34440: F, t34444: F, t7867: F) -> (F, F, F, F, F, F) {
    let t34451 = t7318 * t938;
    let t34455 = t32152 * t6441;
    let t34458 = t7195 * t34434;
    let t34461 = t32233 * t6449;
    let t34468 = t32242 * t930;
    let t34472 = t378 * t938;
    let t34476 = t1642 * t925;
    let t34477 = t32140 * t34476;
    let t34480 = -0.19762785756235085044e-4 * t7867 * t2035 * t34451 - 0.39601100101559655353e-5 * t22796 * t34455 - 0.68116566383613497688e-3 * t22819 * t34458 + t32181 + 0.11352761063935582948e-3 * t22513 * t34461 + 0.25845121844514357744e-4 * t32304 * t34440 + 0.22227677429409423704e-2 * t22623 * t34444 - 0.68246728907663312894e-4 * t32239 * t32241 * t34468 - 0.17608347349624143343e-1 * t32169 * t32140 * t34472 + t32313 + 0.39129660776942540761e-2 * t32279 * t34477;
    (t34461, t34468, t34472, t34476, t34477, t34480)
}
