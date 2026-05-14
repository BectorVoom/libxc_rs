//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1110/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1110<F: Float>(t238: F, t108502: F, t108570: F, t108632: F, t108676: F, t108735: F, t108782: F, t108841: F, t108893: F, t108949: F, t109012: F, t109059: F, t109112: F, t109158: F, t109204: F, t109257: F, t109317: F) -> (F,) {
    let t239 = 0.1e-59 < t238;
    let t109322 = piecewise3(t239, t108502 + t108570 + t108632 + t108676 + t108735 + t108782 + t108841 + t108893 + t108949 + t109012 + t109059 + t109112 + t109158 + t109204 + t109257 + t109317, 0.0);
    (t109322,)
}
