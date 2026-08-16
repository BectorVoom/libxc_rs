//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2051/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2051(t98146: f64, t5622: f64, t94443: f64, t13769: f64, t240: f64, t2661: f64, t7269: f64, t13760: f64, t25972: f64, t5609: f64, t7028: f64, t9845: f64) -> (f64, f64, f64, f64, f64) {
    let t98147 = 0.16006300097412701803e-1_f64 * t98146;
    let t98148 = t94443 * t5622;
    let t98152 = t2661 * t7269 * t240 * t13769;
    let t98156 = t25972 * t13760;
    let t98157 = 0.2032800112371413129e-3_f64 * t98156;
    let t98161 = t9845 * t7028 * t5609;
    (t98147, t98148, t98152, t98157, t98161)
}
