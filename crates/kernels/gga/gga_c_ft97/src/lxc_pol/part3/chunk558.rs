//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 558/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk558<F: Float>(t2481: F, t3908: F, t3925: F, t462: F, t5099: F, t5102: F, t5106: F, t5110: F, t5114: F, t5118: F, t92: F, t734: F, t91: F, t2533: F, t3688: F, t3710: F, t4920: F, t4924: F, t4928: F, t4932: F, t4937: F, t5056: F, t5094: F) -> (F, F, F) {
    let t5120 = t2481 + 2.0 / 9.0 * t3908 + 2.0 / 3.0 * t3925 - 2.0 / 9.0 * t462 * t5099 + 2.0 / 3.0 * t462 * t5102 + 2.0 / 3.0 * t462 * t5106 - t462 * t5110 / 3.0 + 2.0 * t92 * t5114 - t92 * t5118;
    let t5122 = t91 * t734 * t5120;
    let t5132 = -t5094 / 12.0 + t5122 / 6.0 + t2533 + 2.0 / 27.0 * t3688 + 2.0 / 9.0 * t3710 - 2.0 / 27.0 * t4920 + 2.0 / 9.0 * t4924 + 2.0 / 9.0 * t4928 - t4932 / 9.0 + 2.0 / 3.0 * t4937 - t5056 / 3.0;
    (t5120, t5122, t5132)
}
