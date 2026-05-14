//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1067/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1067<F: Float>(t27841: F, t684: F, t24432: F, t6118: F, t24543: F, t27768: F, t192: F, t2506: F, t24477: F, t27819: F, t6878: F, t108168: F, t108172: F, t108176: F, t108182: F, t108191: F, t108195: F, t108200: F, t108204: F) -> (F, F, F, F, F) {
    let t108206 = t27841 * t684;
    let t108208 = t6118 * t24432 * t108206;
    let t108210 = t24543 * t27768;
    let t108211 = 2.0 / 9.0 * t108210;
    let t108212 = t192 * t2506;
    let t108215 = t27819 * t108212 * t6878 * t24477;
    let t108217 = 2.0 * t108168 - t108172 + t108176 / 2.0 - t108182 / 9.0 + t108191 / 4.0 + t108195 / 3.0 + t108200 / 8.0 - t108204 / 6.0 - 2.0 / 3.0 * t108208 + t108211 + 3.0 / 2.0 * t108215;
    (t108206, t108208, t108210, t108215, t108217)
}
