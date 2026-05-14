//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1044/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1044<F: Float>(t1366: F, t3281: F, t38953: F, t5857: F, t5882: F, t8232: F, t95100: F, t95177: F, t95225: F, t95228: F, t95242: F, t95301: F, t95330: F, t95368: F, t95377: F, t5953: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t95859 = 28.0 / 81.0 * t3281 * t1366;
    let t95954 = t38953 * t5857;
    let t95975 = t8232 * t5882;
    let t96064 = 2.0 / 27.0 * t95100;
    let t96083 = 8.0 / 9.0 * t95177;
    let t96099 = 4.0 / 9.0 * t95225;
    let t96100 = t95228 / 9.0;
    let t96104 = 4.0 / 9.0 * t95242;
    let t96119 = 4.0 / 27.0 * t95301;
    let t96130 = 2.0 / 9.0 * t95330;
    let t96140 = 14.0 / 81.0 * t95368;
    let t96143 = 28.0 / 81.0 * t95377;
    let t96160 = t8232 * t5953;
    (t95859, t95954, t95975, t96064, t96083, t96099, t96100, t96104, t96119, t96130, t96140, t96143, t96160)
}
