//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 843/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk843<F: Float>(t25462: F, t6219: F, t317: F, t6260: F, t684: F, t2665: F, t2409: F, t6217: F, t25360: F, t312: F, t10235: F, t2: F, t4: F, t26: F) -> (F, F, F, F, F, F, F) {
    let t25463 = t25462 * t6219;
    let t25465 = t6260 * t317;
    let t25466 = t25465 * t684;
    let t25467 = t2665 * t25466;
    let t25471 = t2665 * t6217 * t2409;
    let t25474 = t25360 * t312;
    let t25478 = t10235 * t2;
    let t25479 = t25478 * t4;
    let t25480 = t25479 * t26;
    (t25463, t25465, t25467, t25471, t25474, t25479, t25480)
}
