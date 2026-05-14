//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1241/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1241<F: Float>(t28741: F, t684: F, t99528: F, t99529: F, t113430: F, t113434: F, t113439: F, t113443: F, t113447: F, t113450: F, t99509: F, t99525: F, t99789: F, t99793: F, t99794: F, t10631: F, t1900: F, t6: F, t91: F) -> (F, F, F) {
    let t113453 = t99528 * t99529 * t28741 * t684;
    let t113455 = t113430 / 4.0 + 12.0 * t113434 + 3.0 / 4.0 * t113439 - t113443 - t99789 - t99793 - t99794 - 4.0 / 9.0 * t99509 + t99525 + t113447 - t113450 + t113453 / 3.0;
    let t113458 = t91 * t10631 * t6 * t1900;
    (t113453, t113455, t113458)
}
