//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1068/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1068(t3937: f64, t3938: f64, t3936: f64, t159: f64, t550: f64, t216: f64) -> (f64, f64, f64) {
    let t3939 = t3937 * t3938;
    let t3940 = t3936 * t3939;
    let t3943 = t159 * t550;
    let t3944 = t216 * t3943;
    (t3940, t3943, t3944)
}
