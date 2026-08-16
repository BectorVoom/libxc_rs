//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 812/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk812(t10914: f64, t2365: f64, t28669: f64, t2021: f64, t7372: f64, t9816: f64, t28152: f64, t787: f64, t9824: f64, t12656: f64, t2684: f64, t7354: f64) -> (f64, f64, f64, f64, f64) {
    let t41463 = t10914 * t2365 * t28669;
    let t41466 = t2021 * t9816 * t7372;
    let t41468 = t787 * t28152;
    let t41469 = t41468 * t9824;
    let t41474 = t2684 * t7354 * t12656;
    (t41463, t41466, t41468, t41469, t41474)
}
