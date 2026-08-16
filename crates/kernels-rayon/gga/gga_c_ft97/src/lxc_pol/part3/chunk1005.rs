//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1005/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1005(t19240: f64, t319: f64, t840: f64, t5299: f64, t882: f64, t4299: f64, t992: f64, t2882: f64, t2881: f64, t15191: f64, t4146: f64, t5393: f64, t870: f64) -> (f64, f64, f64, f64, f64) {
    let t19555 = t840 * t319 * t19240;
    let t19559 = t840 * t882 * t5299;
    let t19563 = t992 * t4299;
    let t19564 = t2882 * t19563;
    let t19565 = t2881 * t19564;
    let t19568 = t15191 * t4146;
    let t19571 = t870 * t5393;
    (t19555, t19559, t19565, t19568, t19571)
}
