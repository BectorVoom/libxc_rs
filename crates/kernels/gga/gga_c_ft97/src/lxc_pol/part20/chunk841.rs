//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 841/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk841<F: Float>(t1476: F, t880: F, t2665: F, t684: F, t2413: F, t6217: F, t10409: F, t2405: F, t6209: F, t92: F) -> (F, F, F, F, F) {
    let t25446 = t1476 * t880;
    let t25448 = t2665 * t25446 * t684;
    let t25452 = t2665 * t6217 * t2413;
    let t25456 = t10409 * t6217 * t2405;
    let t25459 = t6209 * t92;
    (t25446, t25448, t25452, t25456, t25459)
}
