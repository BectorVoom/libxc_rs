//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1176/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1176(t1112: f64, t745: f64, t27618: f64, t343: f64, t810: f64, t8961: f64, t2074: f64, t3178: f64, t2118: f64, t8913: f64, t2352: f64, t8589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27691 = t1112 * t745;
    let t27823 = t27618 * t343;
    let t28024 = t8961 * t810;
    let t28029 = t3178 * t2074;
    let t28139 = t2118 * t8913;
    let t28457 = t8589 * t2352;
    (t27691, t27823, t28024, t28029, t28139, t28457)
}
