//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 849/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk849<F: Float>(t12680: F, t3420: F, t4805: F, t604: F, t379: F, t2210: F, t12724: F, t16150: F, t12723: F, t16169: F, t3440: F, t3439: F) -> (F, F, F, F) {
    let t17195 = t12680 * t3420;
    let t17198 = t604 * t4805;
    let t17199 = t17198 * t379;
    let t17200 = t2210 * t17199;
    let t17203 = t12724 * t16150;
    let t17204 = t12723 * t17203;
    let t17207 = t3440 * t16169;
    let t17208 = t3439 * t17207;
    (t17195, t17200, t17204, t17208)
}
