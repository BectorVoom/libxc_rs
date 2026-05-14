//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1144/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1144<F: Float>(t110052: F, t110065: F, t110079: F, t110091: F, t110105: F, t110118: F, t110132: F, t110144: F, t110158: F, t110171: F, t110185: F, t110198: F, t110211: F, t110224: F, t110237: F, t110250: F) -> (F,) {
    let t110254 = t110052 + t110065 + t110079 + t110091 + t110105 + t110118 + t110132 + t110144 + t110158 + t110171 + t110185 + t110198 + t110211 + t110224 + t110237 + t110250;
    (t110254,)
}
