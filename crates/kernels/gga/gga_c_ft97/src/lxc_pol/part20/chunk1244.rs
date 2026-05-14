//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1244/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1244<F: Float>(t1091: F, t25178: F, t99528: F, t99529: F, t113462: F, t113466: F, t113469: F, t113472: F, t113476: F, t113481: F, t113484: F, t113487: F, t113491: F, t113495: F, t113499: F, t505: F, t856: F) -> (F, F, F) {
    let t113503 = t99528 * t99529 * t1091 * t25178;
    let t113505 = t113462 / 4.0 + t113466 - t113469 / 3.0 - 2.0 / 9.0 * t113472 + t113476 / 6.0 + t113481 / 8.0 + t113484 - t113487 / 12.0 - t113491 / 18.0 - t113495 / 6.0 + t113499 / 3.0 + t113503 / 3.0;
    let t113508 = t856 * t505;
    (t113503, t113505, t113508)
}
