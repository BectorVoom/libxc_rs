//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1008/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1008<F: Float>(t1526: F, t38308: F, t4406: F, t2988: F, t7705: F, t15579: F, t45751: F, t17486: F, t604: F, t2178: F, t4790: F, t2035: F, t39: F, t4673: F, t1354: F, t16785: F) -> (F, F, F, F, F, F, F) {
    let t61184 = t1526 * t38308 * t4406;
    let t61197 = t1526 * t7705 * t2988 / 18.0;
    let t61199 = t1526 * t45751 * t15579;
    let t61330 = t17486 * t604;
    let t61366 = t4790 * t2178;
    let t61607 = t4673 * t39 * t2035;
    let t61631 = t16785 * t1354;
    (t61184, t61197, t61199, t61330, t61366, t61607, t61631)
}
