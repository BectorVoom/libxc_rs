//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1014/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1014<F: Float>(t75912: F, t75949: F, t4702: F, t554: F, t4710: F, t538: F, t1013: F, t3404: F, t135: F, t4674: F, t1526: F, t20514: F, t7705: F, t20518: F, t11262: F, t20507: F) -> (F, F, F, F, F, F, F, F, F) {
    let t75950 = t75912 + t75949;
    let t76883 = t4702 * t554;
    let t76887 = t4710 * t538;
    let t76891 = t4710 * t554;
    let t77125 = t1013 * t3404;
    let t77143 = t4674 * t135;
    let t78650 = t1526 * t7705 * t20514;
    let t78653 = t1526 * t7705 * t20518;
    let t78678 = t1526 * t11262 * t20507;
    (t75950, t76883, t76887, t76891, t77125, t77143, t78650, t78653, t78678)
}
