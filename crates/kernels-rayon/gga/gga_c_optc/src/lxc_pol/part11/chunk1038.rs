//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1038/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1038(t224: f64, t2263: f64, t23573: f64, t23682: f64, t216: f64, t2371: f64, t2414: f64, t24021: f64, t256: f64, t23801: f64, t243: f64, t2491: f64, t2516: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24657 = 1.0_f64 / t224 / t2263;
    let t24658 = t24657 * t23573;
    let t24678 = 0.18467901234567901234e0_f64 * t23682;
    let t24699 = t216 / t2414 / t2371;
    let t24733 = t256 * t24021;
    let t24776 = 0.17757530864197530864e0_f64 * t23682;
    let t24795 = t256 * t23801;
    let t24804 = t243 / t2516 / t2491;
    (t24658, t24678, t24699, t24733, t24776, t24795, t24804)
}
