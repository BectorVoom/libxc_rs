//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 872/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk872<F: Float>(t2224: F, t23443: F, t1651: F, t5942: F, t2210: F, t1643: F, t3439: F, t1384: F, t2178: F) -> (F, F, F, F, F, F) {
    let t23444 = t23443 * t2224;
    let t23447 = t5942 * t1651;
    let t23448 = t2210 * t23447;
    let t23451 = t5942 * t1643;
    let t23452 = t3439 * t23451;
    let t23455 = t2178 * t1384;
    (t23444, t23447, t23448, t23451, t23452, t23455)
}
