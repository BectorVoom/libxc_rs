//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2986/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2986(t14159: f64, t3964: f64, t9285: f64, t213: f64, t225: f64, t46475: f64, t10019: f64, t14114: f64, t14145: f64, t2482: f64, t4114: f64, t5658: f64) -> (f64, f64, f64, f64) {
    let t49432 = t3964 * t14159 * t9285;
    let t49439 = t213 * t225 * t46475;
    let t49446 = t14114 * t10019;
    let t49450 = t2482 * t4114 * t5658 * t14145;
    (t49432, t49439, t49446, t49450)
}
