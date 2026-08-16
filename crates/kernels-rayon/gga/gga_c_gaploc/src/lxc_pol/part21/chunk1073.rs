//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1073/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1073(t1022: f64, t5501: f64, t835: f64, t8720: f64, t2975: f64, t5679: f64, t1710: f64, t2158: f64, t3039: f64, t783: f64, t8633: f64, t1835: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24321 = t5501 * t1022;
    let t24339 = t835 * t8720;
    let t24344 = t5679 * t2975;
    let t24350 = t1022 * t1710;
    let t24364 = t3039 * t2158;
    let t24390 = t8633 * t783;
    let t24446 = t1022 * t1835;
    (t24321, t24339, t24344, t24350, t24364, t24390, t24446)
}
