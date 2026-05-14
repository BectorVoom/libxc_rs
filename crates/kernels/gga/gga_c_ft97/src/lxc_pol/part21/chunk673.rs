//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 673/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk673<F: Float>(t15927: F, t1787: F, t15768: F, t3134: F, t15763: F, t3127: F, t15936: F, t2: F, t4495: F, t1587: F, t432: F, t15625: F, t464: F, t463: F, t4436: F, t7750: F) -> (F, F, F, F, F, F, F, F) {
    let t16378 = t1787 * t15927;
    let t16381 = t3134 * t15768;
    let t16384 = t3127 * t15763;
    let t16387 = t1787 * t15936;
    let t16390 = t2 * t4495;
    let t16392 = t1587 * t16390 * t432;
    let t16395 = t464 * t15625;
    let t16396 = t463 * t16395;
    let t16399 = t2 * t4436;
    let t16401 = t7750 * t16399 * t432;
    (t16378, t16381, t16384, t16387, t16392, t16395, t16396, t16401)
}
