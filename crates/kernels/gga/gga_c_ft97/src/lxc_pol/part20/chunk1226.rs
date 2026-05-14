//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1226/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1226<F: Float>(t3051: F, t6307: F, t18: F, t24981: F, t6334: F, t856: F, t192: F, t2781: F, t25178: F, t28735: F, t7062: F, t113169: F, t113173: F, t113177: F, t113181: F, t113186: F, t113193: F, t113196: F, t113199: F, t113202: F, t113206: F) -> (F, F, F) {
    let t113208 = t6307 * t3051;
    let t113212 = t113208 * t24981 * t6334 * t18 * t856;
    let t113214 = t192 * t2781;
    let t113217 = t28735 * t113214 * t7062 * t25178;
    let t113219 = -t113169 + 2.0 * t113173 - t113177 - 3.0 / 8.0 * t113181 + t113186 / 2.0 - 4.0 / 9.0 * t113193 + t113196 - 2.0 / 3.0 * t113199 + t113202 - t113206 / 12.0 + t113212 / 3.0 + 3.0 / 2.0 * t113217;
    (t113212, t113217, t113219)
}
