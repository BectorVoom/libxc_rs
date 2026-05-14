//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 967/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk967<F: Float>(t21446: F, t7290: F, t2530: F, t701: F, t2610: F, t1835: F, t935: F, t1878: F, t481: F, t941: F, t325: F, t7112: F, t61: F, t98: F) -> (F, F, F, F, F, F, F, F) {
    let t21451 = t7290 * t21446;
    let t21455 = t2530 * t701;
    let t21456 = t2610 * t21455;
    let t21460 = t935 * t1835;
    let t21461 = t2610 * t21460;
    let t21476 = t481 * t941 * t1878;
    let t21483 = t325 * t7112;
    let t21488 = t61 * t98;
    (t21451, t21455, t21456, t21460, t21461, t21476, t21483, t21488)
}
