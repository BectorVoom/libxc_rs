//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1243/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1243<F: Float>(t24981: F, t2756: F, t28735: F, t6334: F, t992: F, t112671: F, t6317: F, t99559: F, t2413: F, t24980: F, t7062: F, t2405: F, t28772: F, t25044: F, t28729: F, t28776: F, t684: F, t99528: F, t99529: F) -> (F, F, F, F, F, F) {
    let t113481 = t28735 * t24981 * t6334 * t992 * t2756;
    let t113484 = t6317 * t99559 * t112671;
    let t113487 = t24980 * t24981 * t7062 * t2413;
    let t113491 = t24980 * t28772 * t7062 * t2405;
    let t113495 = t24980 * t24981 * t25044 * t28729;
    let t113499 = t99528 * t99529 * t28776 * t684;
    (t113481, t113484, t113487, t113491, t113495, t113499)
}
